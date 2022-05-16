mod api;
mod config;
mod services;

#[macro_use]
extern crate rocket;
extern crate envconfig;
extern crate envconfig_derive;
extern crate core;

use std::error::Error;
use crate::envconfig::Envconfig;
use crate::services::download::download_queue::DownloadQueue;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use crate::publisher::Publisher;
use crate::services::{publisher, worker};
use crate::services::file_downloader::FileDownloader;
use crate::services::subscriber::Subscriber;
use crate::worker::Worker;

#[tokio::main]
async fn main() {
    let cfg: Arc<config::Config> = Arc::new(config::Config::init().unwrap());
    let publisher = Arc::new(Publisher::new().await.unwrap());
    let subscriber = Subscriber::new().await.unwrap();
    let download_queue = Arc::new(DownloadQueue::new(cfg.clone(), publisher.clone()));
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
            })
                .unwrap(),
        )
        .unwrap();


    sched.start().unwrap();

    let mut worker = Arc::new(Worker::new(subscriber, publisher.clone(), file_downloader.clone()));

    let (_,_) = tokio::join!(
        worker.run(),
        rocket::build()
            .attach(api::stage())
            .attach(api::download::stage(download_queue, publisher.clone(), cfg.clone()))
            .launch()
    );
}
