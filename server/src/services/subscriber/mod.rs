use std::error::Error;
use std::future::Future;
use std::io::ErrorKind;
use std::sync::Arc;
use ratsio::{NatsClient, NatsClientOptions};
use futures::StreamExt;
use ratsio::ops::Message;

pub struct Subscriber {
    conn: Arc<NatsClient>,
}

impl Subscriber {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let conn = NatsClient::new("localhost:4222").await?;

        Ok(Self { conn })
    }

    pub(crate) async fn run<F, Fut>(&self, handler: F) -> Result<(), Box<dyn Error>>
        where
            F: Fn(Message) -> Fut,
            Fut: Future<Output=()>
    {
        let (_, mut sub) = self.conn.subscribe_with_group("darklight.download", "darklight-workers").await?;

        while let Some(msg) = sub.next().await {
            handler(msg).await
        }

        Ok(())
    }
}
