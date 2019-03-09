mod login;
mod mutation;
mod project;
mod query;
mod submission;
mod tokenized;
mod user;
pub use self::login::*;
pub use self::mutation::*;
pub use self::project::*;
pub use self::query::*;
pub use self::submission::*;
pub use self::tokenized::*;
pub use self::user::*;

use crate::db::Connection;
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
