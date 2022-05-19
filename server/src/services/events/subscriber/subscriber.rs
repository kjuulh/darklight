use std::error::Error;
use std::future::Future;
use std::sync::Arc;
use futures::StreamExt;
use ratsio::{NatsClient};
use ratsio::ops::Message;

use crate::envconfig::Envconfig;

#[derive(Envconfig)]
pub struct SubscriberCfg {
    #[envconfig(from = "NATS_URL", default = "localhost:4222")]
    pub nats_url: String,
}

pub struct Subscriber {
    conn: Arc<NatsClient>,
}

impl Subscriber {
    pub async fn new(cfg: Arc<SubscriberCfg>) -> Result<Self, Box<dyn Error>> {
        let conn = NatsClient::new(cfg.nats_url.as_str()).await?;

        Ok(Self { conn })
    }

    pub(crate) async fn run<F, Fut>(&self, subject: &str, group: Option<&str>, handler: F) -> Result<(), Box<dyn Error>>
        where
            F: Fn(Message) -> Fut,
            Fut: Future<Output=()>
    {
        if let Some(g) = group {
            let (_, mut sub) = self.conn.subscribe_with_group(subject, g).await?;

            while let Some(msg) = sub.next().await {
                handler(msg).await
            }
        } else {
            let (_, mut sub) = self.conn.subscribe(subject).await?;

            while let Some(msg) = sub.next().await {
                handler(msg).await
            }
        };


        Ok(())
    }
}
