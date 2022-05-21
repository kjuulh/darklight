extern crate envconfig;
extern crate envconfig_derive;
#[macro_use]
extern crate rocket;

use std::error::Error;
use std::sync::Arc;

use darklight_app::download_queue::DownloadQueue;

use crate::api_config::ApiConfig;
use crate::envconfig::Envconfig;

mod health_check;
mod download;
pub mod api_config;

pub struct ApiDependencies {
    cfg: Arc<ApiConfig>,
    download_queue: Arc<DownloadQueue>,
}

impl ApiDependencies {
    pub fn new(cfg: Arc<ApiConfig>, download_queue: Arc<DownloadQueue>) -> Self {
        Self {
            cfg,
            download_queue,
        }
    }

    pub fn new_from_env(download_queue: Arc<DownloadQueue>) -> Result<Self, Box<dyn Error>> {
        let api_cfg = Arc::new(api_config::ApiConfig::init_from_env()?);
        Ok(Self::new(api_cfg, download_queue))
    }
}

pub async fn build(deps: ApiDependencies) -> Result<(), Box<dyn Error>> {
    match rocket::build()
        .attach(health_check::stage())
        .attach(download::stage(deps.download_queue.clone(), deps.cfg.clone()))
        .launch().await {
        Ok(_) => { Ok(()) }
        Err(e) => { Err(e.into()) }
    }
}

