use std::error::Error;
use std::sync::Arc;

use ratsio::NatsClient;
use serde::Serialize;

use crate::envconfig::Envconfig;

#[derive(Envconfig)]
pub struct PublisherCfg {
    #[envconfig(from = "NATS_URL", default = "localhost:4222")]
    pub nats_url: String,
}

pub struct Publisher {
    conn: Arc<NatsClient>,
}

impl Publisher {
    pub async fn new(cfg: Arc<PublisherCfg>) -> Result<Self, Box<dyn Error>> {
        let conn = NatsClient::new(cfg.nats_url.as_str()).await?;

        Ok(Self { conn })
    }

    pub async fn new_from_env() -> Result<Self, Box<dyn Error>> {
        let publisher_cfg = Arc::new(PublisherCfg::init_from_env()?);
        Self::new(publisher_cfg).await
    }

    pub async fn publish<T>(&self, subject: &str, payload: T) -> Result<(), Box<dyn Error>>
        where
            T: Serialize {
        let payload = serde_json::to_string(&payload)?;

        self.conn.publish(subject, payload.as_bytes()).await?;

        Ok(())
    }
}
