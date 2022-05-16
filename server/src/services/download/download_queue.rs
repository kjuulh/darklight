use rocket::{
    tokio::sync::{Mutex},
    fs::NamedFile,
    tokio,
    tokio::task,
};
use std::{
    collections::{HashMap},
    error::{Error},
    path::PathBuf,
    sync::Arc,
};
use std::fmt::format;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Serializer, Deserialize, Deserializer, de};
use crate::{config::Config, Publisher, services::yt::{Arg, YoutubeDL}};
use chrono::serde::ts_milliseconds;

#[derive(Clone)]
pub enum DownloadState {
    Initiated,
    Downloading,
    Done,
    Error,
}

impl Serialize for DownloadState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(match self {
            DownloadState::Initiated => "initiated",
            DownloadState::Downloading => "downloading",
            DownloadState::Done => "done",
            DownloadState::Error => "error",
        })
    }
}

impl<'de> Deserialize<'de> for DownloadState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?.to_lowercase();
        let state = match s.as_str() {
            "initiated" => DownloadState::Initiated,
            "downloading" => DownloadState::Downloading,
            "done" => DownloadState::Done,
            "error" => DownloadState::Error,
            other => { return Err(de::Error::custom(format!("Invalid state '{}'", other))); }
        };

        Ok(state)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Download {
    pub id: String,
    pub state: DownloadState,
    pub link: String,
    pub file: Option<String>,
    #[serde(with = "ts_milliseconds")]
    pub insert_time: DateTime<Utc>,
}

pub struct DownloadQueue {
    downloads: Arc<Mutex<HashMap<String, Download>>>,
    cfg: Arc<Config>,
    publisher: Arc<Publisher>,
}

impl DownloadQueue {
    pub fn new(cfg: Arc<Config>, publisher: Arc<Publisher>) -> Self {
        let config = cfg.clone();
        task::spawn(async move {
            tokio::fs::remove_dir_all(config.storage_path.to_string()).await
        });

        Self {
            cfg: cfg.clone(),
            downloads: Arc::new(Mutex::new(HashMap::new())),
            publisher,
        }
    }

    pub async fn add(&self, link: &'_ str) -> Result<String, Box<dyn Error>> {
        let download_id = Uuid::new_v4().to_string();

        let downloads = self.downloads.clone();
        let mut locked_downloads = downloads.lock().await;

        let download = Download {
            id: download_id.clone(),
            state: DownloadState::Initiated,
            link: link.to_string(),
            file: None,
            insert_time: Utc::now(),
        };
        locked_downloads.insert(
            download_id.clone(),
            download.clone(),
        );

        self.publisher.publish("darklight.download".into(), &download).await?;

        locked_downloads
            .get_mut(&download_id)
            .map(|download| {
                download.state = DownloadState::Done;
                download
            });

        Ok(download_id)
    }

    pub async fn get(&self, download_id: &'_ str) -> Option<Download> {
        self.downloads
            .lock()
            .await
            .get(download_id).cloned()
    }

    pub async fn get_file(&self, download_id: &'_ str) -> Option<NamedFile> {
        if let Some(download) = self.downloads.lock().await.get(download_id) {
            let mut dir = tokio::fs::read_dir(format!("{}/{}", self.cfg.storage_path, download.id)).await.ok()?;

            if let Some(entry) = dir.next_entry().await.ok()? {
                return NamedFile::open(entry.path()).await.ok();
            }
        }

        None
    }

    pub async fn remove_old(&self) -> Result<(), Box<dyn Error>> {
        println!("remove old files triggered");
        let mut downloads = self.downloads.lock().await;

        for download in downloads.clone().iter().map(|d| d.1) {
            if is_older(download.insert_time, Utc::now()) {
                println!("cleaning up for: {}", download.id);
                match self.clean_up(download).await {
                    Ok(_) => {
                        println!("cleanup done for: {}", download.id);

                        match downloads.remove(download.id.as_str()) {
                            None => {
                                println!("Could not fine download")
                            }
                            Some(_) => {
                                println!("removed from db")
                            }
                        }
                    }
                    Err(e) => {
                        println!("cleanup failed: {:?}", e)
                    }
                }
            }
        }

        Ok(())
    }

    async fn clean_up(&self, download: &Download) -> std::io::Result<()> {
        tokio::fs::remove_dir_all(format!("{}/{}", self.cfg.storage_path, download.id)).await
    }
}

fn is_older(created: DateTime<Utc>, now: DateTime<Utc>) -> bool {
    created + chrono::Duration::minutes(5) < now
}


#[cfg(test)]
mod tests {
    use chrono::Utc;
    use crate::services::download::download_queue::is_older;

    #[test]
    fn datetime() {
        let older = is_older(Utc::now() - chrono::Duration::minutes(5) - chrono::Duration::seconds(1), Utc::now());

        assert_eq!(older, true)
    }
}