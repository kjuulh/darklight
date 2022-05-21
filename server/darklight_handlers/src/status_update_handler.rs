use std::{
    error::Error,
    sync::Arc,
};

use darklight_events::events;
use darklight_events::models::DownloadStatus;
use darklight_events::subscriber::subscriber::Subscriber;
use darklight_persistence::repos::downloads::DownloadRepo;

use crate::utility::parse_to_str;

pub struct StatusUpdateHandler {
    subscriber: Arc<Subscriber>,
    download_repo: Arc<DownloadRepo>,
}

impl StatusUpdateHandler {
    pub fn new(subscriber: Arc<Subscriber>, download_repo: Arc<DownloadRepo>) -> Self {
        Self { subscriber, download_repo }
    }

    pub async fn run(self: Arc<Self>) {
        if let Err(e) = self.subscriber.run(events::DOWNLOAD_UPDATE, Some(events::DOWNLOAD_UPDATE_GROUP), |msg| {
            let s = Arc::clone(&self);
            async move {
                if let Err(e) = s.run_done_downloading(&msg.payload).await {
                    eprintln!("failed to run done downloading: {}", e)
                }
            }
        }).await {
            eprintln!("{}", e)
        }
    }

    async fn run_done_downloading(&self, payload: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        let download = parse_to_str(payload).and_then(serialize_download)?;
        println!("Finished download: {}", download.download_id);

        self.download_repo.update_percentage(download.download_id, download.percentage).await?;
        println!("Finished download, database updated: {}", download.download_id);

        Ok(())
    }
}

fn serialize_download(payload: &str) -> Result<DownloadStatus, Box<dyn Error>> {
    match serde_json::from_str::<DownloadStatus>(payload) {
        Ok(d) => Ok(d),
        Err(e) => Err(e.into())
    }
}
