use std::sync::Arc;

use darklight_app::file_downloader::FileDownloader;
use darklight_events::publisher::Publisher;
use darklight_events::subscriber::subscriber::Subscriber;
use darklight_persistence::repos::downloads::DownloadRepo;
use darklight_storage::storage_uploader::FileUploader;

use crate::done_downloading_handler::DoneDownloadingHandler;
use crate::download_worker::DownloadWorker;
use crate::file_name_available_handler::FileNameAvailableHandler;
use crate::status_update_handler::StatusUpdateHandler;

pub mod download_worker;
pub mod done_downloading_handler;
pub mod status_update_handler;
pub mod file_name_available_handler;
mod utility;

//let external_queue = download_queue.clone();
//let sched = JobScheduler::new().unwrap();
//sched
//.add(
//Job::new_async("1/30 * * * * *", move |_uuid, _l| {
//let external_queue = external_queue.clone();
//Box::pin(async move {
//external_queue.remove_old().await.unwrap();
//})
//}).unwrap(),
//).unwrap();


//sched.start().unwrap();

pub struct HandlerDependencies {
    subscriber: Arc<Subscriber>,
    publisher: Arc<Publisher>,
    file_downloader: Arc<FileDownloader>,
    file_uploader: Arc<FileUploader>,
    download_repo: Arc<DownloadRepo>,
}

impl HandlerDependencies {
    pub fn new(
        subscriber: Arc<Subscriber>,
        publisher: Arc<Publisher>,
        file_downloader: Arc<FileDownloader>,
        file_uploader: Arc<FileUploader>,
        download_repo: Arc<DownloadRepo>,
    ) -> Self {
        Self {
            subscriber,
            publisher,
            file_downloader,
            file_uploader,
            download_repo,
        }
    }
}

pub async fn run_handlers(deps: HandlerDependencies) {
    let download_worker = Arc::new(DownloadWorker::new(deps.subscriber.clone(), deps.publisher.clone(), deps.file_downloader.clone(), deps.file_uploader.clone()));
    let done_downloading_handler = Arc::new(DoneDownloadingHandler::new(deps.subscriber.clone(), deps.download_repo.clone()));
    let status_update_handler = Arc::new(StatusUpdateHandler::new(deps.subscriber.clone(), deps.download_repo.clone()));
    let file_name_available_handler = Arc::new(FileNameAvailableHandler::new(deps.subscriber.clone(), deps.download_repo.clone()));

    let _ = tokio::join!(
        download_worker.run(),
        done_downloading_handler.run(),
        status_update_handler.run(),
        file_name_available_handler.run(),
    );
}
