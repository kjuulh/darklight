use std::error::Error;
use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;

use darklight_core::download::Download;
use darklight_events::events;
use darklight_events::models::{DownloadFileNameAvailable, DownloadStatus};
use darklight_events::publisher::Publisher;
use darklight_ytd::youtube_dl::{Arg, YoutubeDL};

use crate::envconfig::Envconfig;

#[derive(Envconfig)]
pub struct FileDownloaderCfg {
    #[envconfig(from = "STORAGE_PATH", default = "./target/output")]
    pub storage_path: String,
}

pub struct FileDownloader {
    pub cfg: Arc<FileDownloaderCfg>,
    publisher: Arc<Publisher>,
}

impl FileDownloader {
    pub fn new(cfg: Arc<FileDownloaderCfg>, publisher: Arc<Publisher>) -> Self {
        Self {
            cfg,
            publisher,
        }
    }

    pub fn new_from_env(publisher: Arc<Publisher>) -> Result<Self, Box<dyn Error>> {
        let file_downloader_cfg = Arc::new(FileDownloaderCfg::init_from_env()?);

        Ok(Self::new(file_downloader_cfg, publisher))
    }

    pub async fn download(&self, download: &Download) -> Result<String, Box<dyn Error>> {
        if let Err(e) = download_media(
            self.cfg.storage_path.to_string(),
            download.link.as_str(),
            download.id.as_ref().unwrap().as_str(),
            |percentage| {
                async move {
                    if let Err(e) = self.publisher.publish(events::DOWNLOAD_UPDATE, DownloadStatus::new(download.id.as_ref().unwrap().as_str(), percentage)).await {
                        eprintln!("{}", e)
                    }
                }
            },
            |file_name| {
                async move {
                    if let Err(e) = self.publisher.publish(events::DOWNLOAD_FILE_NAME_AVAILABLE, DownloadFileNameAvailable::new(download.id.as_ref().unwrap().as_str(), file_name)).await {
                        eprintln!("{}", e)
                    }
                }
            },
        ).await {
            println!("{}", e);
            return Err("failure".into());
        }

        let mut dir = tokio::fs::read_dir(format!("{}/{}", self.cfg.storage_path, download.id.as_ref().unwrap())).await.unwrap();

        let file_name = dir.next_entry().await.map(|entry| entry.map(|e| e.file_name().to_string_lossy().to_string()));

        if let Ok(Some(f)) = file_name {
            println!("downloaded: {}", f);
            Ok(f)
        } else {
            Err("could not download file".into())
        }
    }


    pub async fn get_file(&self, download_id: &'_ str) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut dir = tokio::fs::read_dir(format!("{}/{}", self.cfg.storage_path, download_id)).await?;

        if let Some(entry) = dir.next_entry().await? {
            return FileDownloader::read_file(entry.path()).await;
        }

        Err("could not find file".into())
    }

    async fn read_file(file_path: PathBuf) -> Result<Vec<u8>, Box<dyn Error>> {
        match tokio::fs::read(file_path).await {
            Ok(f) => Ok(f),
            Err(e) => Err(e.into())
        }
    }
}

async fn download_media<F, Fut, FAvailable, FutAvailable>(storage_path: String, link: &'_ str, id: &'_ str, progress_update_fn: F, file_name_available: FAvailable) -> Result<(), Box<dyn Error>>
    where
        F: Fn(u32) -> Fut,
        FAvailable: Fn(String) -> FutAvailable,
        Fut: Future<Output=()>,
        FutAvailable: Future<Output=()> {
    let args = vec![
//Arg::new("--quiet"),
Arg::new("--progress"),
Arg::new("--newline"),
Arg::new_with_args("--output", "%(title).90s.%(ext)s"),
    ];

    let path = PathBuf::from(format!("{storage_path}/{id}"));
    let ytd = YoutubeDL::new(&path, args, link)?;

// start download
    let download = ytd.download(progress_update_fn, file_name_available).await?;

// print out the download path
    println!("Your download: {}", download.output_dir().to_string_lossy());
    Ok(())
}

