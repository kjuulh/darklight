use std::error::Error;
use async_graphql::{Context, FieldResult, ID, Object, Result, SimpleObject};
use crate::GraphQLDependencies;

pub struct MutationRoot;

#[derive(SimpleObject)]
struct RequestDownloadResp {
    id: ID,
}

#[Object]
impl MutationRoot {
    async fn request_download(&self, ctx: &Context<'_>, link: String) -> Result<RequestDownloadResp> {
        match ctx.data_unchecked::<GraphQLDependencies>()
            .download_queue.add(link.as_str())
            .await {
            Ok(id) => Ok(RequestDownloadResp { id: ID::from(id) }),
            Err(e) => Err(async_graphql::Error::new(e.to_string()))
        }
    }
}