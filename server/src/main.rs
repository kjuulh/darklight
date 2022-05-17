extern crate core;
extern crate envconfig;
extern crate envconfig_derive;
#[macro_use]
extern crate rocket;

use std::sync::Arc;

use dotenv::dotenv;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::envconfig::Envconfig;
use crate::publisher::Publisher;
use crate::services::{publisher, worker};
use crate::services::download::download_queue::DownloadQueue;
use crate::services::file_downloader::FileDownloader;
use crate::services::file_uploader::{FileUploader, FileUploaderCfg};
use crate::services::subscriber::Subscriber;
use crate::worker::Worker;

mod api;
mod config;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cfg: Arc<config::Config> = Arc::new(config::Config::init_from_env().unwrap());
    let file_uploader_cfg = FileUploaderCfg::init_from_env().unwrap();
    let file_uploader = Arc::new(FileUploader::new(file_uploader_cfg).await.unwrap());
    let publisher = Arc::new(Publisher::new().await.unwrap());
    let subscriber = Subscriber::new().await.unwrap();
    let download_queue = Arc::new(DownloadQueue::new(cfg.clone(), publisher.clone(), file_uploader.clone()));
    let file_downloader = Arc::new(FileDownloader::new(cfg.clone()));

    let external_queue = download_queue.clone();
    let sched = JobScheduler::new().unwrap();
    sched
        .add(
            Job::new_async("1/30 * * * * *", move |_uuid, _l| {
                let external_queue = external_queue.clone();
                Box::pin(async move {
                    external_queue.remove_old().await.unwrap();
                })
            }).unwrap(),
        ).unwrap();


    sched.start().unwrap();

    let worker = Arc::new(Worker::new(subscriber, publisher.clone(), file_downloader.clone(), file_uploader.clone()));

    let (_, _) = tokio::join!(
        worker.run(),
        rocket::build()
            .attach(api::stage())
            .attach(api::download::stage(download_queue,cfg.clone()))
            .launch()
    );
}
