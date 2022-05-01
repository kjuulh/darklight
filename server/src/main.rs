mod api;
mod services;

#[macro_use]
extern crate rocket;

use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use crate::services::download::download_queue::{DownloadQueue, DownloadState};

#[rocket::main]
async fn main() {
    let download_queue = Arc::new(DownloadQueue::new());


    let external_queue = download_queue.clone();
    let mut sched = JobScheduler::new().unwrap();
    sched.add(Job::new_async("1/30 * * * * *", move |uuid, l| {
        let external_queue = external_queue.clone();
        Box::pin(async move {
            external_queue.remove_old().await;
        })
    }).unwrap());

    sched.start().unwrap();

    rocket::build()
        .attach(api::download::stage(download_queue))
        .launch()
        .await;
}
