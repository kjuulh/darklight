extern crate core;
extern crate envconfig;
extern crate envconfig_derive;

use std::sync::Arc;

use dotenv::dotenv;

use darklight_api::ApiDependencies;
use darklight_app::download_queue::DownloadQueue;
use darklight_app::file_downloader::FileDownloader;
use darklight_events::publisher::Publisher;
use darklight_events::subscriber::subscriber::Subscriber;
use darklight_handlers::HandlerDependencies;
use darklight_persistence::repos::downloads::DownloadRepo;
use darklight_storage::storage_downloader::S3StorageDownloader;
use darklight_storage::storage_uploader::FileUploader;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let download_repo = Arc::new(DownloadRepo::new_postgres().await.unwrap());
    let file_uploader = Arc::new(FileUploader::new_from_env().await.unwrap());
    let s3_storage_downloader = Arc::new(S3StorageDownloader::new_from_env().await.unwrap());
    let publisher = Arc::new(Publisher::new_from_env().await.unwrap());
    let subscriber = Arc::new(Subscriber::new_from_env().await.unwrap());
    let download_queue = Arc::new(DownloadQueue::new_from_env(publisher.clone(), download_repo.clone(), s3_storage_downloader.clone()).unwrap());
    let file_downloader = Arc::new(FileDownloader::new_from_env(publisher.clone()).unwrap());
    let handler_deps = HandlerDependencies::new(subscriber.clone(), publisher.clone(), file_downloader.clone(), file_uploader.clone(), download_repo.clone());
    let api_deps = ApiDependencies::new_from_env(download_queue.clone()).unwrap();

    let _ = tokio::join!(
        darklight_handlers::run_handlers(handler_deps),
        darklight_api::build(api_deps)
    );
}
