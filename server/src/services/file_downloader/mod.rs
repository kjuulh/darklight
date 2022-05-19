use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

use crate::config::Config;
use crate::services::download::download::Download;
use crate::services::yt::{Arg, YoutubeDL};

pub struct FileDownloader {
    pub cfg: Arc<Config>,
}

impl FileDownloader {
    pub fn new(cfg: Arc<Config>) -> Self {
        Self {
            cfg
        }
    }

    pub async fn download(&self, download: &Download) -> Result<String, Box<dyn Error>> {
        if let Err(e) = download_media(self.cfg.storage_path.to_string(), download.link.as_str(), download.id.as_ref().unwrap().as_str()).await {
            println!("{}", e);
            return Err("failure".into());
        }

        let mut dir = tokio::fs::read_dir(format!("{}/{}", self.cfg.storage_path, download.id.as_ref().unwrap())).await.unwrap();

        let file_name = dir.next_entry().await.and_then(|entry| {
            Ok(entry.and_then(|e| {
                Some(e.file_name().to_string_lossy().to_string())
            }))
        });

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

async fn download_media(storage_path: String, link: &'_ str, id: &'_ str) -> Result<(), Box<dyn Error>> {
    let args = vec![
        //Arg::new("--quiet"),
        Arg::new("--progress"),
        Arg::new("--newline"),
        Arg::new_with_args("--output", "%(title).90s.%(ext)s"),
    ];

    let path = PathBuf::from(format!("{storage_path}/{id}"));
    let ytd = YoutubeDL::new(&path, args, link)?;

    // start download
    let download = ytd.download().await?;

    // print out the download path
    println!("Your download: {}", download.output_dir().to_string_lossy());
    Ok(())
}

