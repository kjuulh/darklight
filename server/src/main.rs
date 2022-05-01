mod api;
mod services;
mod config;

#[macro_use]
extern crate rocket;
extern crate envconfig_derive;
extern crate envconfig;

use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use crate::services::download::download_queue::{DownloadQueue};
use crate::envconfig::Envconfig;

#[rocket::main]
async fn main() {
    let cfg: Arc<config::Config> = Arc::new(config::Config::init().unwrap());

    let download_queue = Arc::new(DownloadQueue::new(cfg.clone()));

    let external_queue = download_queue.clone();
    let sched = JobScheduler::new().unwrap();
    sched.add(Job::new_async("1/30 * * * * *", move |_uuid, _l| {
        let external_queue = external_queue.clone();
        Box::pin(async move {
            external_queue.remove_old().await.unwrap();
        })
    }).unwrap()).unwrap();

    sched.start().unwrap();

    rocket::build()
        .attach(api::download::stage(download_queue, cfg.clone()))
        .launch()
        .await
        .unwrap();
}
