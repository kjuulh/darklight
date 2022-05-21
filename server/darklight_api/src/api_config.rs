use crate::envconfig::Envconfig;

#[derive(Envconfig)]
pub struct ApiConfig {
    #[envconfig(from = "FRONTEND_URL", default = "http://localhost:3000")]
    pub frontend_url: String,

}