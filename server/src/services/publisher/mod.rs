use std::error::Error;
use std::sync::Arc;
use ratsio::NatsClient;
use serde::{Deserialize, Serialize};

pub struct Publisher {
    conn: Arc<NatsClient>,
}

impl Publisher {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let conn = NatsClient::new("localhost:4222").await?;

        Ok(Self { conn })
    }

    pub async fn publish<T>(&self, subject: String, payload: T) -> Result<(), Box<dyn Error>>
        where
            T: Serialize {
        let payload = serde_json::to_string(&payload)?;

        self.conn.publish(subject, payload.as_bytes()).await?;

        Ok(())
    }
}
