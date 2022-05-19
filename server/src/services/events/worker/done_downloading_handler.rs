use std::{
    error::Error,
    sync::Arc,
};
use crate::{
    services::{
        events::{
            events,
            models::DoneDownloading,
            worker::utility::parse_to_str,
        }
    },
    DownloadRepo,
    Subscriber,
};

pub struct DoneDownloadingHandler {
    subscriber: Arc<Subscriber>,
    download_repo: Arc<DownloadRepo>,
}

impl DoneDownloadingHandler {
    pub fn new(subscriber: Arc<Subscriber>, download_repo: Arc<DownloadRepo>) -> Self {
        Self { subscriber, download_repo }
    }

    pub async fn run(self: Arc<Self>) {
        if let Err(e) = self.subscriber.run(events::DOWNLOAD_DONE, Some(events::DONE_DOWNLOADING_GROUP), |msg| {
            let s = Arc::clone(&self);
            async move {
                match parse_to_str(&msg.payload).and_then(serialize_download) {
                    Ok(payload) => {
                        println!("Finished download: {}", payload.download_id);
                        match s.download_repo.finish_download(payload.download_id).await {
                            Ok(_) => {
                                println!("Finished download, database updated: {}", payload.download_id);
                            }
                            Err(e) => {
                                eprint!("Failed to update database: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprint!("Failed to parse done download event: {}", e);
                    }
                }
            }
        }).await {
            eprintln!("{}", e)
        }
    }
}

fn serialize_download(payload: &str) -> Result<DoneDownloading, Box<dyn Error>> {
    match serde_json::from_str::<DoneDownloading>(payload) {
        Ok(d) => Ok(d),
        Err(e) => Err(e.into())
    }
}
