use std::sync::Arc;

use rocket::{Request, response, State};
use rocket::fairing::AdHoc;
use rocket::fs::NamedFile;
use rocket::http::{Header, Method};
use rocket::response::Responder;
use rocket::response::status::NotFound;
use rocket::serde::{Deserialize, json::Json, Serialize};
use rocket_cors::AllowedOrigins;

use crate::config::Config;
use crate::Publisher;
use crate::services::download::download::Download;
use crate::services::download::download_queue::DownloadQueue;
use crate::services::download::download_state::DownloadState;

type Downloads<'r> = &'r State<Arc<DownloadQueue>>;
type NatsPublisher<'r> = &'r State<Arc<Publisher>>;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct DownloadRequest<'r> {
    link: &'r str,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct DownloadResponse {
    id: String,
    link: String,
    state: String,
    file_name: Option<String>,
}

impl From<Download> for DownloadResponse {
    fn from(download: Download) -> Self {
        Self {
            id: download.id,
            state: match download.state {
                DownloadState::Initiated => "initiated".into(),
                DownloadState::Downloading => "downloading".into(),
                DownloadState::Done => "done".into(),
                DownloadState::Error => "error".into(),
            },
            link: download.link,
            file_name: download.file,
        }
    }
}

#[post("/", format = "json", data = "<download_request>")]
async fn request_download(
    publisher: NatsPublisher<'_>,
    download_request: Json<DownloadRequest<'_>>,
) -> String {
    if let Err(_) = publisher.publish("darklight.download".into(), download_request.0).await {
        return "FAILED".into();
    }

    return "OK".into();
}

#[get("/<download_id>")]
async fn get_request_download(
    download_id: &str,
    downloads: Downloads<'_>,
) -> Result<Json<DownloadResponse>, NotFound<String>> {
    match downloads.get(download_id).await {
        Some(download) => Ok(Json(download.into())),
        None => Err(NotFound("could not find download".into())),
    }
}

struct DownloadedFile {
    file: NamedFile,
}

impl<'r> Responder<'r, 'static> for DownloadedFile {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let file_path = self.file.path().file_name().unwrap().to_string_lossy().to_string();
        let mut response = self.file.respond_to(req)?;

        response.set_header(Header::new("Content-Disposition", format!("attachment; filename=\"{}\"", file_path)));

        Ok(response)
    }
}

#[get("/<download_id>/file")]
async fn get_downloaded_file(
    download_id: &str,
    downloads: Downloads<'_>,
) -> Result<DownloadedFile, NotFound<String>> {
    match downloads.get_file(download_id).await {
        Some(download) => Ok(DownloadedFile { file: download }),
        None => Err(NotFound("could not find download".into())),
    }
}

pub fn stage(
    download_queue: Arc<DownloadQueue>,
    publisher: Arc<Publisher>,
    cfg: Arc<Config>,
) -> AdHoc {
    let allowed_origins = AllowedOrigins::some_exact(&[cfg.frontend_url.to_string()]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Post].into_iter().map(From::from).collect(),
        ..Default::default()
    }.to_cors();

    AdHoc::on_ignite("downloads", |rocket| async {
        rocket
            .mount("/download", routes![request_download, get_request_download, get_downloaded_file])
            .manage(download_queue)
            .manage(publisher)
            .attach(cors.unwrap())
    })
}
