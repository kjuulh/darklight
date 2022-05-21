use std::error::Error;

use s3::{Bucket, Region};
use s3::creds::Credentials;

use crate::envconfig::Envconfig;

#[derive(Envconfig)]
pub struct FileUploaderCfg {
    #[envconfig(from = "MINIO_ACCESS_KEY")]
    pub access_key: String,

    #[envconfig(from = "MINIO_SECRET")]
    pub secret: String,

    #[envconfig(from = "MINIO_URL")]
    pub url: String,
}

pub struct FileUploader {
    bucket: Bucket,
}

struct Storage {
    region: Region,
    credentials: Credentials,
    bucket: String,
}

impl FileUploader {
    pub async fn new(cfg: FileUploaderCfg) -> Result<Self, Box<dyn Error>> {
        let storage = FileUploader::connect(&cfg)?;
        let bucket = FileUploader::create_bucket(&storage)?;

        if let Err(_) = bucket.put_object("somefile.txt", "some-file".as_bytes()).await {
            panic!("{}", "could not put test file in bucket")
        }

        Ok(Self {
            bucket,
        })
    }

    pub async fn new_from_env() -> Result<Self, Box<dyn Error>> {
        let file_uploader_cfg = FileUploaderCfg::init_from_env()?;
        Self::new(file_uploader_cfg).await
    }

    fn connect(cfg: &FileUploaderCfg) -> Result<Storage, Box<dyn Error>> {
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

    pub async fn upload(&self, filename: String, file: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        match self.bucket.put_object(format!("{}", filename), file).await {
            Ok((_, 200)) => Ok(()),
            Ok((_, _)) => Err("failed to upload file".into()),
            Err(e) => Err(e.into()),
        }
    }
}
