use std::{
    collections::HashMap,
    error::Error,
    sync::Arc,
};

use chrono::{
    DateTime,
    Utc,
};
use rocket::{
    tokio,
    tokio::sync::Mutex,
    tokio::task,
};
use uuid::Uuid;

use crate::{config::Config, DownloadRepo, Publisher, services::download::download_state::DownloadState};
use crate::services::download::download::Download;
use crate::services::events::events;

pub struct DownloadQueue {
    downloads: Arc<Mutex<HashMap<String, Download>>>,
    cfg: Arc<Config>,
    publisher: Arc<Publisher>,
    download_repo: Arc<DownloadRepo>,
}

impl DownloadQueue {
    pub fn new(cfg: Arc<Config>, publisher: Arc<Publisher>, download_repo: Arc<DownloadRepo>) -> Self {
        let config = cfg.clone();
        task::spawn(async move {
            tokio::fs::remove_dir_all(config.storage_path.to_string()).await
        });

        Self {
            cfg: cfg.clone(),
            downloads: Arc::new(Mutex::new(HashMap::new())),
            publisher,
            download_repo,
        }
    }

    pub async fn add(&self, link: &'_ str) -> Result<String, Box<dyn Error>> {
        let download = Download {
            id: None,
            state: DownloadState::Initiated,
            link: link.to_string(),
            file: None,
            insert_time: Some(Utc::now()),
        };

        let download = self.download_repo.add_download(&download).await?;
        self.publisher.publish(events::DOWNLOADS, &download).await?;

        match download.id {
            None => Err("download was not created properly".into()),
            Some(id) => Ok(id),
        }
    }

    pub async fn get(&self, download_id: &'_ str) -> Result<Option<Download>, Box<dyn Error>> {
        self.download_repo.get_by_download_id(download_id).await
    }

    pub async fn remove_old(&self) -> Result<(), Box<dyn Error>> {
        println!("remove old files triggered");
        let mut downloads = self.downloads.lock().await;

        for download in downloads.clone().iter().map(|d| d.1) {
            if is_older(download.insert_time.unwrap(), Utc::now()) {
                println!("cleaning up for: {}", download.id.as_ref().unwrap());
                match self.clean_up(download).await {
                    Ok(_) => {
                        println!("cleanup done for: {}", download.id.as_deref().unwrap());

                        match downloads.remove(download.id.as_ref().unwrap().as_str()) {
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
        tokio::fs::remove_dir_all(format!("{}/{}", self.cfg.storage_path, download.id.as_ref().unwrap())).await
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