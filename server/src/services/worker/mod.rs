use std::error::Error;
use std::sync::Arc;
use crate::Publisher;
use crate::services::download::download::Download;
use crate::services::file_downloader::FileDownloader;
use crate::services::subscriber::Subscriber;

pub struct Worker {
    subscriber: Subscriber,
    publisher: Arc<Publisher>,
    file_downloader: Arc<FileDownloader>,
}

impl Worker {
    pub fn new(subscriber: Subscriber, publisher: Arc<Publisher>, file_downloader: Arc<FileDownloader>) -> Self {
        Self { subscriber, publisher, file_downloader }
    }

    pub async fn run(self: Arc<Self>) {
        if let Err(e) = self.subscriber.run(|msg| {
            let s = Arc::clone(&self);
            async move {
                match parse_to_str(&msg.payload).and_then(serialize_download) {
                    Ok(payload) => {
                        if let Err(e) = s.file_downloader.download(&payload).await {
                            eprintln!("{}", e)
                        }
                    }
                    Err(e) => {
                        eprint!("{}", e.to_string());
                    }
                }
            }
        }).await {
            eprintln!("{}", e)
        }
    }
}

fn parse_to_str<'a>(payload: &'a Vec<u8>) -> Result<&'a str, Box<dyn Error>> {
    match std::str::from_utf8(payload.as_slice()) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.into())
    }
}

fn serialize_download(payload: &str) -> Result<Download, Box<dyn Error>> {
    match serde_json::from_str::<Download>(payload) {
        Ok(d) => Ok(d),
        Err(e) => Err(e.into())
    }
}