use std::error::Error;
use std::future::Future;
use std::sync::Arc;

use futures::{Stream, StreamExt};
use ratsio::{NatsClient, NatsClientOptions, RatsioError};
use ratsio::ops::{Message, Op};

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

    pub async fn new_from_env() -> Result<Self, Box<dyn Error>> {
        let subscriber_cfg = Arc::new(SubscriberCfg::init_from_env()?);
        Self::new(subscriber_cfg).await
    }

    pub async fn run<F, Fut>(&self, subject: &str, group: Option<&str>, handler: F) -> Result<(), Box<dyn Error>>
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

    pub async fn get_stream(&self, subject: String) -> impl Stream<Item=Message> + Send + Sync {
        let (_, mut sub) = self.conn.subscribe(subject).await.unwrap();
        sub
    }
}
