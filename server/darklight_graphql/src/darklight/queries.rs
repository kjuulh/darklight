use async_graphql::{Context, Object, SimpleObject, Result, ID, Number, Error};
use darklight_app::download_queue::DownloadQueue;
use crate::GraphQLDependencies;

#[derive(SimpleObject)]
pub struct Download {
    pub id: ID,
    pub state: String,
    pub link: String,
    pub file: Option<String>,
    pub percentage: u32,
}

impl TryFrom<darklight_core::download::Download> for Download {
    type Error = async_graphql::Error;
    fn try_from(d: darklight_core::download::Download) -> std::result::Result<Self, Self::Error> {
        if let None = d.id {
            return Err("id is missing from download".into());
        }

        Ok(Self {
            id: ID::from(d.id.unwrap()),
            state: d.state.as_str().to_string(),
            link: d.link,
            file: d.file,
            percentage: d.percentage,
        })
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello_world(&self) -> &str {
        "Hello, world!"
    }

    async fn get_download(&self, ctx: &Context<'_>, download_id: ID) -> Result<Option<Download>> {
        match ctx.data_unchecked::<GraphQLDependencies>().download_queue.get(download_id.as_str()).await {
            Ok(Some(d)) => match d.try_into() {
                Ok(d) => Ok(Some(d)),
                Err(e) => Err(e)
            },
            Ok(None) => Ok(None),
            Err(e) => Err(async_graphql::Error::new(e.to_string()))
        }
    }
}