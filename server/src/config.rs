use std::env;
use crate::envconfig::Envconfig;


#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "FRONTEND_URL", default = "http://localhost:3000")]
    pub frontend_url: String,

    #[envconfig(from = "STORAGE_PATH", default = "./target/output")]
    pub storage_path: String,
}