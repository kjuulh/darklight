use chrono::{DateTime, serde::ts_milliseconds_option, Utc};
use serde::{Deserialize, Serialize};

use crate::services::download::download_state::DownloadState;

#[derive(Clone, Serialize, Deserialize)]
pub struct Download {
    pub id: Option<String>,
    pub state: DownloadState,
    pub link: String,
    pub file: Option<String>,
    #[serde(with = "ts_milliseconds_option")]
    pub insert_time: Option<DateTime<Utc>>,
    pub percentage: u32,
}