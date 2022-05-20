use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use crate::PostgresDb;
use crate::services::download::download::Download;
use crate::services::download::download_state::DownloadState;

pub struct DownloadRepo {
    db: Arc<PostgresDb>,
}

impl DownloadRepo {
    pub fn new(db: Arc<PostgresDb>) -> Self {
        Self {
            db
        }
    }

    pub async fn add_download(&self, download: &Download) -> Result<Download, Box<dyn Error>> {
        let mut conn = self.db.pool.acquire().await?;
        let rec = sqlx::query_file!(
            "src/services/database/repos/downloads/add_download.sql",
            download.state.to_string(),
            download.link,
            download.file,
            download.insert_time,
        )
            .fetch_one(&mut conn)
            .await?;

        let mut new_download = download.clone();
        new_download.id = Some(rec.download_id.to_string());

        return Ok(new_download);
    }

    pub async fn finish_download(&self, download_id: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
        let mut conn = self.db.pool.acquire().await?;

        let _ = sqlx::query_file!(
            "src/services/database/repos/downloads/finish_download.sql",
            DownloadState::Done.to_string(),
            file_name,
            sqlx::types::Uuid::from_str(download_id)?
        )
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn get_by_download_id(&self, download_id: &str) -> Result<Option<Download>, Box<dyn Error>> {
        let mut conn = self.db.pool.acquire().await?;
        let rec = sqlx::query_file!(
            "src/services/database/repos/downloads/get_download_by_download_id.sql",
            sqlx::types::Uuid::from_str(download_id)?
        )
            .fetch_optional(&mut conn)
            .await?;

        match rec {
            None => {
                Ok(None)
            }
            Some(d) => {
                let download = Download {
                    id: Some(d.download_id.to_string()),
                    state: DownloadState::from_string(d.state.as_str()).expect("download_state should always be set"),
                    link: d.link,
                    file: d.file,
                    insert_time: Some(d.insert_time)
                };

                return Ok(Some(download));
            }
        }
    }
}