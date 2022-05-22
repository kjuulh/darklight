use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

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

impl DownloadState {
    pub fn as_str(&self) -> &str {
        match self {
            DownloadState::Initiated => "initiated",
            DownloadState::Downloading => "downloading",
            DownloadState::Done => "done",
            DownloadState::Error => "error",
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        let state = match s {
            "initiated" => DownloadState::Initiated,
            "downloading" => DownloadState::Downloading,
            "done" => DownloadState::Done,
            "error" => DownloadState::Error,
            _ => { return None; }
        };

        Some(state)
    }
}