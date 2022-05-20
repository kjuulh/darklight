use std::sync::Arc;

use rocket::{
    response::{
        status::NotFound,
        Responder,
        self,
    },
    http::{Header, Method},
    fs::NamedFile,
    fairing::AdHoc,
    Request,
    State,
    serde::{Deserialize, json::Json, Serialize},
};
use rocket_cors::AllowedOrigins;

use crate::{
    config::Config,
    services::download::{
        download::Download,
        download_queue::DownloadQueue,
        download_state::DownloadState,
    },
};

type Downloads<'r> = &'r State<Arc<DownloadQueue>>;

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
    percentage: u32,
}

impl From<Download> for DownloadResponse {
    fn from(download: Download) -> Self {
        Self {
            id: download.id.unwrap(),
            state: match download.state {
                DownloadState::Initiated => "initiated".into(),
                DownloadState::Downloading => "downloading".into(),
                DownloadState::Done => "done".into(),
                DownloadState::Error => "error".into(),
            },
            link: download.link,
            file_name: download.file,
            percentage: download.percentage,
        }
    }
}

#[post("/", format = "json", data = "<download_request>")]
async fn request_download(
    downloads: Downloads<'_>,
    download_request: Json<DownloadRequest<'_>>,
) -> String {
    match downloads.add(download_request.link).await {
        Ok(download_id) => download_id,
        Err(e) => format!("{}", e)
    }
}

#[get("/<download_id>")]
async fn get_request_download(
    download_id: &str,
    downloads: Downloads<'_>,
) -> Result<Json<DownloadResponse>, NotFound<String>> {
    match downloads.get(download_id).await {
        Ok(Some(download)) => Ok(Json(download.into())),
        Ok(None) => Err(NotFound("could not find download".into())),
        Err(e) => panic!("{}", e),
    }
}

struct DownloadedFile {
    file_name: String,
    file_data: Vec<u8>,
}

impl<'r> Responder<'r, 'static> for DownloadedFile {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let mut response = self.file_data.respond_to(req)?;
        response.set_header(Header::new("Content-Disposition", format!("attachment; filename=\"{}\"", self.file_name)));
        Ok(response)
    }
}

#[get("/<download_id>/file")]
async fn get_downloaded_file<'a>(
    download_id: &'a str,
    downloads: Downloads<'a>,
) -> Result<DownloadedFile, NotFound<String>> {
    match downloads.get_file(download_id).await {
        Ok(Some((file_name, file_data))) => Ok(DownloadedFile { file_name, file_data }),
        Ok(None) => Err(NotFound("could not find download".into())),
        Err(..) => Err(NotFound("could not find download".into())),
    }
}

pub fn stage(
    download_queue: Arc<DownloadQueue>,
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
            .mount("/api/download", routes![request_download, get_request_download, get_downloaded_file])
            .manage(download_queue)
            .attach(cors.unwrap())
    })
}
