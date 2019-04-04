mod event;
mod login;
mod medium;
mod mutation;
mod permission;
mod project;
mod query;
mod role;
mod submission;
mod token;
mod tokenized;
mod user;
mod user_event;
mod user_role;
pub use self::event::*;
pub use self::login::*;
pub use self::medium::*;
pub use self::mutation::*;
pub use self::permission::*;
pub use self::project::*;
pub use self::query::*;
pub use self::role::*;
pub use self::submission::*;
pub use self::token::*;
pub use self::tokenized::*;
pub use self::user::*;
pub use self::user_event::*;
pub use self::user_role::*;

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
