use chrono::{DateTime, serde::ts_milliseconds, Utc};
use serde::{Deserialize, Serialize};

use crate::services::download::download_state::DownloadState;

#[derive(Clone, Serialize, Deserialize)]
pub struct Download {
    pub id: String,
    pub state: DownloadState,
    pub link: String,
    pub file: Option<String>,
    #[serde(with = "ts_milliseconds")]
    pub insert_time: DateTime<Utc>,
}
