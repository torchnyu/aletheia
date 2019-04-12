//! The graphql module contains the code necessary for GraphQL integration
//! with Aletheia. All that should be necessary is `use crate::graphql::*;`
//! and then you should be good to go.

// GraphQL done with juniper

// There should be an easier way to do this.
mod event;
mod mutation;
mod project;
mod query;
mod tokenized;
mod user;

pub use event::*;
pub use mutation::*;
pub use project::*;
pub use query::*;
pub use tokenized::*;
pub use user::*;

use juniper::Context as JuniperContext;
use juniper::RootNode;
pub use crate::db::connection::RequestContext;

pub fn create_schema() -> Schema {
    RootNode::new(QueryRoot {}, MutationRoot {})
}

impl JuniperContext for RequestContext {}

type Schema = RootNode<'static, QueryRoot, MutationRoot>;
