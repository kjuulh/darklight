use std::borrow::BorrowMut;
use rocket::tokio::sync::Mutex;
use std::collections::HashMap;
use std::error::Error;
use std::ops::Add;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use chrono::{DateTime, Utc};
use rocket::fs::NamedFile;
use rocket::log::private::log;
use rocket::tokio;
use rocket::tokio::io::AsyncReadExt;
use rocket::tokio::task;
use uuid::Uuid;
use ytd_rs::{Arg, YoutubeDL};

#[derive(Clone)]
pub enum DownloadState {
    Initiated,
    Downloading,
    Done,
    Error,
}

#[derive(Clone)]
pub struct Download {
    pub id: String,
    pub state: DownloadState,
    pub link: String,
    pub file: Option<String>,
    insert_time: DateTime<Utc>,
}

pub struct DownloadQueue {
    downloads: Arc<Mutex<HashMap<String, Download>>>,
}

impl DownloadQueue {
    pub fn new() -> Self {
        task::spawn(async {
            tokio::fs::remove_dir_all(format!("./target/output")).await
        });

        Self {
            downloads: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add(&self, link: &'_ str) -> String {
        let download_id = Uuid::new_v4().to_string();

        let downloads = self.downloads.clone();
        let mut locked_downloads = downloads.lock().await;

        locked_downloads.insert(
            download_id.clone(),
            Download {
                id: download_id.clone(),
                state: DownloadState::Initiated,
                link: link.to_string(),
                file: None,
                insert_time: Utc::now(),
            },
        );

        if let Err(e) = download_media(link, download_id.as_str()) {
            println!("{}", e);
            return "failure".into();
        }

        let mut file_name: Option<String> = None;
        let mut dir = tokio::fs::read_dir(format!("./target/output/{}", download_id)).await.unwrap();
        if let Some(entry) = dir.next_entry().await.unwrap() {
            file_name = Some(entry.file_name().to_string_lossy().to_string());
        }

        locked_downloads
            .get_mut(&download_id)
            .and_then(|download| {
                download.state = DownloadState::Done;
                download.file = file_name;
                Some(download)
            });

        download_id
    }

    pub async fn get(&self, download_id: &'_ str) -> Option<Download> {
        self.downloads
            .lock()
            .await
            .get(download_id)
            .and_then(|download| Some(download.clone()))
    }

    pub async fn get_file(&self, download_id: &'_ str) -> Option<NamedFile> {
        if let Some(download) = self.downloads.lock().await.get(download_id) {
            let mut dir = tokio::fs::read_dir(format!("./target/output/{}", download.id)).await.ok()?;

            if let Some(entry) = dir.next_entry().await.ok()? {
                return NamedFile::open(entry.path()).await.ok();
            }
        }

        return None;
    }

    pub async fn remove_old(&self) -> Result<(), Box<dyn Error>> {
        println!("remove old files triggered");
        let mut downloads = self.downloads.lock().await;

        for download in downloads.iter().map(|d| d.1) {
            if is_older(download.insert_time, Utc::now()) {
                println!("cleaning up for: {}", download.id);
                match self.clean_up(download).await {
                    Ok(_) => {
                        println!("cleanup done for: {}", download.id)
                    }
                    Err(e) => {
                        println!("cleanup failed: {:?}", e)
                    }
                }
            }
        }

        return Ok(());
    }

    async fn clean_up(&self, download: &Download) -> std::io::Result<()> {
        tokio::fs::remove_dir_all(format!("./target/output/{}", download.id)).await
    }
}

fn is_older(created: DateTime<Utc>, now: DateTime<Utc>) -> bool {
    created + chrono::Duration::minutes(5) < now
}

fn download_media(link: &'_ str, id: &'_ str) -> Result<(), Box<dyn Error>> {
    let args = vec![
        //Arg::new("--quiet"),
        Arg::new_with_arg("--output", "%(title).90s.%(ext)s"),
    ];

    let path = PathBuf::from(format!("./target/output/{id}"));
    let ytd = YoutubeDL::new(&path, args, link)?;

    // start download
    let download = ytd.download()?;

    // print out the download path
    println!("Your download: {}", download.output_dir().to_string_lossy());
    Ok(())
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