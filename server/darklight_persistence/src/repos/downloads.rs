use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;

use darklight_core::download::Download;
use darklight_core::download_state::DownloadState;

use crate::postgres::PostgresDb;

pub struct DownloadRepo {
    db: Arc<PostgresDb>,
}

impl DownloadRepo {
    pub fn new(db: Arc<PostgresDb>) -> Self {
        Self {
            db
        }
    }

    pub async fn new_postgres() -> Result<Self, Box<dyn Error>> {
        let postgres = Arc::new(PostgresDb::new_from_env().await?);
        Ok(Self::new(postgres))
    }

    pub async fn add_download(&self, download: &Download) -> Result<Download, Box<dyn Error>> {
        let mut conn = self.db.pool.acquire().await?;
        let rec = sqlx::query_file!(
            "src/repos/downloads/add_download.sql",
            download.state.as_str(),
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
            "src/repos/downloads/finish_download.sql",
            DownloadState::Done.as_str(),
            file_name,
            sqlx::types::Uuid::from_str(download_id)?
        )
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn update_percentage(&self, download_id: &str, percentage: u32) -> Result<(), Box<dyn Error>> {
        let mut conn = self.db.pool.acquire().await?;


        let _ = sqlx::query_file!(
            "src/repos/downloads/update_percentage.sql",
            i64::from(percentage),
            sqlx::types::Uuid::from_str(download_id)?
        )
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn update_file_name(&self, download_id: &str, file_name: String) -> Result<(), Box<dyn Error>> {
        let mut conn = self.db.pool.acquire().await?;


        let _ = sqlx::query_file!(
            "src/repos/downloads/update_file_name.sql",
            file_name.as_str(),
            sqlx::types::Uuid::from_str(download_id)?
        )
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn get_by_download_id(&self, download_id: &str) -> Result<Option<Download>, Box<dyn Error>> {
        let mut conn = self.db.pool.acquire().await?;
        let rec = sqlx::query_file!(
            "src/repos/downloads/get_download_by_download_id.sql",
            sqlx::types::Uuid::from_str(download_id)?
        )
            .fetch_optional(&mut conn)
            .await?;

        match rec {
            None => {
                Ok(None)
            }
            Some(d) => {
                let p = d.percentage.unwrap() as u32;
                let download = Download {
                    id: Some(d.download_id.to_string()),
                    state: DownloadState::from_string(d.state.as_str()).expect("download_state should always be set"),
                    link: d.link,
                    file: d.file,
                    insert_time: Some(d.insert_time),
                    percentage: p,
                };

                return Ok(Some(download));
            }
        }
    }
}