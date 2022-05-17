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

    pub async fn download(&self, download: &Download) -> Result<(), Box<dyn Error>> {
        if let Err(e) = download_media(self.cfg.storage_path.to_string(), download.link.as_str(), download.id.as_str()).await {
            println!("{}", e);
            return Err("failure".into());
        }

        let mut dir = tokio::fs::read_dir(format!("{}/{}", self.cfg.storage_path, download.id)).await.unwrap();
        let file_name = if let Some(entry) = dir.next_entry().await.unwrap() {
            Some(entry.file_name().to_string_lossy().to_string())
        } else {
            None
        };

        if let Some(f) = file_name {
            println!("downloaded: {}", f)
        }

        Ok(())
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

