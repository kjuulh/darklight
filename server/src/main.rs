extern crate core;
extern crate envconfig;
extern crate envconfig_derive;
#[macro_use]
extern crate rocket;

use std::sync::Arc;

use dotenv::dotenv;
use tokio_cron_scheduler::{Job, JobScheduler};
use services::events::worker;
use services::events::worker::download_worker::DownloadWorker;

use crate::{
    envconfig::Envconfig,
    services::{
        database::{
            postgres::{PostgresDb, PostgresDbCfg},
            repos::downloads::DownloadRepo,
        },
        download::download_queue::DownloadQueue,
        events::{
            publisher::{Publisher, PublisherCfg},
            subscriber::subscriber::{Subscriber, SubscriberCfg},
        },
        file_downloader::FileDownloader,
        file_uploader::{FileUploader, FileUploaderCfg},
    },
};
use crate::services::storage_downloader::s3_storage_downloader::{S3StorageDownloader, S3StorageDownloaderCfg};
use crate::worker::done_downloading_handler::DoneDownloadingHandler;

mod api;
mod config;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cfg: Arc<config::Config> = Arc::new(config::Config::init_from_env().unwrap());
    let postgres_cfg = Arc::new(PostgresDbCfg::init_from_env().unwrap());
    let publisher_cfg = Arc::new(PublisherCfg::init_from_env().unwrap());
    let subscriber_cfg = Arc::new(SubscriberCfg::init_from_env().unwrap());

    let postgres = Arc::new(PostgresDb::new(postgres_cfg).await.unwrap());
    let download_repo = Arc::new(DownloadRepo::new(postgres));
    let file_uploader_cfg = FileUploaderCfg::init_from_env().unwrap();
    let file_uploader = Arc::new(FileUploader::new(file_uploader_cfg).await.unwrap());
    let s3_storage_downloader_cfg = S3StorageDownloaderCfg::init_from_env().unwrap();
    let s3_storage_downloader = Arc::new(S3StorageDownloader::new(s3_storage_downloader_cfg).await.unwrap());
    let publisher = Arc::new(Publisher::new(publisher_cfg.clone()).await.unwrap());
    let subscriber = Arc::new(Subscriber::new(subscriber_cfg.clone()).await.unwrap());
    let download_queue = Arc::new(DownloadQueue::new(cfg.clone(), publisher.clone(), download_repo.clone(), s3_storage_downloader.clone()));
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


    //sched.start().unwrap();

    let download_worker = Arc::new(DownloadWorker::new(subscriber.clone(), publisher.clone(), file_downloader.clone(), file_uploader.clone()));
    let done_downloading_handler = Arc::new(DoneDownloadingHandler::new(subscriber.clone(), download_repo.clone()));

    let (_, _, _) = tokio::join!(
        download_worker.run(),
        done_downloading_handler.run(),
        rocket::build()
            .attach(api::stage())
            .attach(api::download::stage(download_queue,cfg.clone()))
            .launch()
    );
}
