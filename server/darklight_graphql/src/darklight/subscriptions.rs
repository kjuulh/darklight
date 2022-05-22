use std::error::Error;
use async_graphql::{Context, Object, SimpleObject, Result, ID, Number, Subscription};
use async_graphql::futures_util::{Stream, StreamExt, TryStreamExt};
use darklight_app::download_queue::DownloadQueue;
use darklight_events::events::DOWNLOAD_UPDATE;
use darklight_events::models::DownloadStatus;
use crate::darklight::queries;
use crate::GraphQLDependencies;

pub struct SubscriptionRoot;

struct DownloadChanged {
    id: ID,
}

#[Object]
impl DownloadChanged {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn download(&self, ctx: &Context<'_>) -> Result<Option<queries::Download>> {
        match ctx.data_unchecked::<GraphQLDependencies>().download_queue.get(self.id.as_str()).await {
            Ok(Some(d)) => match d.try_into() {
                Ok(d) => Ok(Some(d)),
                Err(e) => Err(e)
            },
            Ok(None) => Ok(None),
            Err(e) => Err(async_graphql::Error::new(e.to_string()))
        }
    }
}


#[Subscription]
impl SubscriptionRoot {
    async fn get_download(&self, ctx: &Context<'_>, download_id: ID) -> impl Stream<Item=DownloadChanged> {
        let stream = ctx.data_unchecked::<GraphQLDependencies>().subscriber.get_stream(DOWNLOAD_UPDATE.to_string()).await;
        let next_stream = StreamExt::filter_map(stream, move |mut msg| {
            let d = download_id.clone();
            async move {
                let payload = msg.payload.as_slice();
                let download = serde_json::from_slice::<DownloadStatus>(payload).unwrap();

                if download.download_id == d.clone().as_str() {
                    Some(DownloadChanged { id: d })
                } else {
                    None
                }
            }
        });

        return next_stream;
    }
}