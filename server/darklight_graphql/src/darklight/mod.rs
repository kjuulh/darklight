mod queries;
mod mutations;
mod subscriptions;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub use queries::QueryRoot;
pub use mutations::MutationRoot;
pub use crate::darklight::subscriptions::SubscriptionRoot;

pub type DarklightSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;