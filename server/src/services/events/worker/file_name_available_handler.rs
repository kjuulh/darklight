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
use crate::services::events::models::{DownloadFileNameAvailable, DownloadStatus};

pub struct FileNameAvailableHandler {
    subscriber: Arc<Subscriber>,
    download_repo: Arc<DownloadRepo>,
}

impl FileNameAvailableHandler {
    pub fn new(subscriber: Arc<Subscriber>, download_repo: Arc<DownloadRepo>) -> Self {
        Self { subscriber, download_repo }
    }

    pub async fn run(self: Arc<Self>) {
        if let Err(e) = self.subscriber.run(events::DOWNLOAD_FILE_NAME_AVAILABLE, Some(events::DOWNLOAD_FILE_NAME_AVAILABLE_GROUP), |msg| {
            let s = Arc::clone(&self);
            async move {
                if let Err(e) = s.update_file_name(&msg.payload).await {
                    eprintln!("failed to update with file name: {}", e)
                }
            }
        }).await {
            eprintln!("{}", e)
        }
    }

    async fn update_file_name(&self, payload: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        let download = parse_to_str(payload).and_then(serialize_download)?;
        println!("Update file name: {}", download.download_id);

        self.download_repo.update_file_name(download.download_id, download.file_name).await?;
        println!("Updated file name, database updated: {}", download.download_id);

        Ok(())
    }
}

fn serialize_download(payload: &str) -> Result<DownloadFileNameAvailable, Box<dyn Error>> {
    match serde_json::from_str::<DownloadFileNameAvailable>(payload) {
        Ok(d) => Ok(d),
        Err(e) => Err(e.into())
    }
}
