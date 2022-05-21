use std::error::Error;

use s3::{Bucket, Region};
use s3::creds::Credentials;

use crate::envconfig::Envconfig;

#[derive(Envconfig)]
pub struct S3StorageDownloaderCfg {
    #[envconfig(from = "MINIO_ACCESS_KEY")]
    pub access_key: String,

    #[envconfig(from = "MINIO_SECRET")]
    pub secret: String,

    #[envconfig(from = "MINIO_URL")]
    pub url: String,
}

struct Storage {
    region: Region,
    credentials: Credentials,
    bucket: String,
}

pub struct S3StorageDownloader {
    bucket: Bucket,
}

impl S3StorageDownloader {
    pub async fn new(cfg: S3StorageDownloaderCfg) -> Result<Self, Box<dyn Error>> {
        let storage = S3StorageDownloader::connect(&cfg)?;
        let bucket = S3StorageDownloader::create_bucket(&storage)?;

        Ok(Self {
            bucket,
        })
    }

    pub async fn new_from_env() -> Result<Self, Box<dyn Error>> {
        let s3_storage_downloader_cfg = S3StorageDownloaderCfg::init_from_env()?;
        Self::new(s3_storage_downloader_cfg).await
    }

    fn connect(cfg: &S3StorageDownloaderCfg) -> Result<Storage, Box<dyn Error>> {
        println!("Bootstrapping Minio storage");
        let minio = Storage {
            region: Region::Custom {
                region: "".into(),
                endpoint: cfg.url.clone(),

            },
            credentials: Credentials {
                access_key: Some(cfg.access_key.clone()),
                secret_key: Some(cfg.secret.clone()),
                security_token: None,
                session_token: None,
            },
            bucket: "downloads".to_string(),
        };

        Ok(minio)
    }
    fn create_bucket(storage: &Storage) -> Result<Bucket, Box<dyn Error>> {
        println!("Creating Minio bucket connection");
        match Bucket::new(&storage.bucket, storage.region.clone(), storage.credentials.clone()) {
            Ok(b) => Ok(b.with_path_style()),
            Err(e) => Err(e.into())
        }
    }

    pub async fn download_file(&self, file_name: &str) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        let (data, code) = self.bucket.get_object(format!("/{}", file_name)).await?;
        if code != 200 {
            Ok(None)
        } else {
            Ok(Some(data))
        }
    }
}
