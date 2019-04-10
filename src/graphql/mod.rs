//! The graphql module contains the code necessary for GraphQL integration
//! with Aletheia. All that should be necessary is `use crate::graphql::*;`
//! and then you should be good to go.

// GraphQL done with juniper

// There should be an easier way to do this.
mod project;
mod tokenized;
mod mutation;
mod event;
mod user;
mod query;

pub use mutation::*;
pub use project::*;
pub use tokenized::*;
pub use event::*;
pub use user::*;
pub use query::*;

pub use crate::db::Connection;
use juniper::Context as JuniperContext;
use juniper::RootNode;

pub fn create_schema() -> Schema {
    RootNode::new(QueryRoot {}, MutationRoot {})
}

pub struct Context {
    pub database: Connection,
}

impl JuniperContext for Context {}

type Schema = RootNode<'static, QueryRoot, MutationRoot>;
