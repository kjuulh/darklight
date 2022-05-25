use std::error::Error;
use std::sync::Arc;

use darklight_app::file_downloader::FileDownloader;
use darklight_core::download::Download;
use darklight_events::events;
use darklight_events::models::DoneDownloading;
use darklight_events::publisher::Publisher;
use darklight_events::subscriber::subscriber::Subscriber;
use darklight_storage::storage_uploader::FileUploader;

use crate::utility::parse_to_str;

pub struct DownloadWorker {
    subscriber: Arc<Subscriber>,
    publisher: Arc<Publisher>,
    file_downloader: Arc<FileDownloader>,
    file_uploader: Arc<FileUploader>,
}

impl DownloadWorker {
    pub fn new(subscriber: Arc<Subscriber>, publisher: Arc<Publisher>, file_downloader: Arc<FileDownloader>, file_uploader: Arc<FileUploader>) -> Self {
        Self { subscriber, publisher, file_downloader, file_uploader }
    }

    pub async fn run(self: Arc<Self>) {
        if let Err(e) = self.subscriber.run(events::DOWNLOADS, Some(events::WORKER_GROUP), |msg| {
            let s = Arc::clone(&self);
            async move {
                match parse_to_str(&msg.payload).and_then(serialize_download) {
                    Ok(payload) => {
                        match s.file_downloader.download(&payload).await {
                            Ok(file_name) => {
                                match s.file_downloader.get_file(payload.id.as_ref().unwrap()).await {
                                    Ok(content) => match s.file_uploader.upload(file_name.clone(), &content).await {
                                        Ok(_) => {
                                            if let Err(e) = s.publisher.publish(events::DOWNLOAD_DONE, DoneDownloading::new(payload.id.as_ref().unwrap().as_str(), file_name.as_str())).await {
                                                eprintln!("failed to publish event: {}", e)
                                            }
                                            println!("succeeded in uploading file")
                                        }
                                        Err(e) => {
                                            eprintln!("{}", e);
                                        }
                                    },
                                    Err(e) => {
                                        eprintln!("{}", e);
                                    }
                                }
                            }

                            Err(e) => {
                                eprintln!("{}", e)
                            }
                        }
                    }
                    Err(e) => {
                        eprint!("{}", e);
                    }
                }
            }
        }).await {
            eprintln!("{}", e)
        }
    }
}

fn serialize_download(payload: &str) -> Result<Download, Box<dyn Error>> {
    match serde_json::from_str::<Download>(payload) {
        Ok(d) => Ok(d),
        Err(e) => Err(e.into())
    }
}
