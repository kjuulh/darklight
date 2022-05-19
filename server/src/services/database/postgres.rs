use std::error::Error;
use std::sync::Arc;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use crate::envconfig::Envconfig;

#[derive(Envconfig)]
pub struct PostgresDbCfg {
    #[envconfig(from = "POSTGRES_CONN")]
    pub postgres_conn: String,
}

pub struct PostgresDb {
    pub pool: PgPool,
}

impl PostgresDb {
    pub async fn new(cfg: Arc<PostgresDbCfg>) -> Result<Self, Box<dyn Error>> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(cfg.postgres_conn.as_str()).await?;

        Ok(Self {
            pool,
        })
    }
}