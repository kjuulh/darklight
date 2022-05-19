use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DoneDownloading<'a> {
    pub download_id: &'a str,
}

impl <'a> DoneDownloading<'a> {
    pub fn new(download_id: &'a str) -> Self {
        Self {
            download_id
        }
    }
}