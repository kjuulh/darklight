use async_graphql::async_stream::stream;
use async_graphql::futures_util::{Stream, StreamExt};
use async_graphql::{Context, Object, Result, Subscription, ID};

use crate::darklight::queries;
use crate::GraphQLDependencies;
use darklight_events::events::DOWNLOAD_UPDATE;
use darklight_events::models::DownloadStatus;

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
        match ctx
            .data_unchecked::<GraphQLDependencies>()
            .download_queue
            .get(self.id.as_str())
            .await
        {
            Ok(Some(d)) => match d.try_into() {
                Ok(d) => Ok(Some(d)),
                Err(e) => Err(e),
            },
            Ok(None) => Ok(None),
            Err(e) => Err(async_graphql::Error::new(e.to_string())),
        }
    }
}

#[Subscription]
impl SubscriptionRoot {
    async fn get_download(
        &self,
        ctx: &Context<'_>,
        download_id: ID,
    ) -> impl Stream<Item = DownloadChanged> {
        let stream = ctx
            .data_unchecked::<GraphQLDependencies>()
            .subscriber
            .get_stream(DOWNLOAD_UPDATE.to_string())
            .await;
        let d_id = download_id.clone();
        let next_stream = StreamExt::filter_map(stream, move |msg| {
            let d = d_id.clone();
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

        let initial_request = stream! {
            yield DownloadChanged { id: download_id.clone() }
        };

        StreamExt::chain(initial_request, next_stream)
    }
}
