use crate::GraphQLDependencies;
use async_graphql::{Context, FieldResult, Object, Result, SimpleObject, ID};
use std::error::Error;

pub struct MutationRoot;

#[derive(SimpleObject)]
struct RequestDownloadResp {
    id: ID,
}

#[Object]
impl MutationRoot {
    async fn request_download(
        &self,
        ctx: &Context<'_>,
        link: String,
        requester_id: ID,
    ) -> Result<RequestDownloadResp> {
        match ctx
            .data_unchecked::<GraphQLDependencies>()
            .download_queue
            .add(link.as_str(), requester_id.0)
            .await
        {
            Ok(id) => Ok(RequestDownloadResp { id: ID::from(id) }),
            Err(e) => Err(async_graphql::Error::new(e.to_string())),
        }
    }
}
