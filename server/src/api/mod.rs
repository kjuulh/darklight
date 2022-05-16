use rocket::fairing::AdHoc;
use rocket_cors::AllowedOrigins;

pub mod download;

#[get("/healthz")]
pub fn get_health_check() -> String {
    "Ok!".into()
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("base", |rocket| async {
        rocket.mount("/", routes![get_health_check])
    })
}
