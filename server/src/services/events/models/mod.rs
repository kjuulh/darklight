use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DoneDownloading<'a> {
    pub download_id: &'a str,
    pub file_name: &'a str,
}

impl<'a> DoneDownloading<'a> {
    pub fn new(download_id: &'a str, file_name: &'a str) -> Self {
        Self {
            download_id,
            file_name,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DownloadStatus<'a> {
    pub download_id: &'a str,
    pub percentage: u32,
}

impl<'a> DownloadStatus<'a> {
    pub fn new(download_id: &'a str, percentage: u32) -> Self {
        Self {
            download_id,
            percentage,
        }
    }
}



#[derive(Serialize, Deserialize)]
pub struct DownloadFileNameAvailable<'a> {
    pub download_id: &'a str,
    pub file_name: String,
}

impl<'a> DownloadFileNameAvailable<'a> {
    pub fn new(download_id: &'a str, file_name: String) -> Self {
        Self {
            download_id,
            file_name,
        }
    }
}



