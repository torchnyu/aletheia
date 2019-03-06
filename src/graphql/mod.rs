pub mod project_type;
pub mod user_type;

use crate::db::Connection;
use crate::models::{LoginRequest, LoginResponse, Project, UserResponse};
use juniper::Context as JuniperContext;
use juniper::FieldResult;
use juniper::RootNode;

pub struct Context {
    pub database: Connection,
}

impl JuniperContext for Context {}

type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    RootNode::new(QueryRoot {}, MutationRoot {})
}

pub struct QueryRoot {}
pub struct MutationRoot {}

graphql_object!(QueryRoot: Context as "Query" |&self| {
    description: "The root query object of the schema"

    field users(
        &executor
    ) -> FieldResult<Vec<UserResponse>> {
        let database = &executor.context().database;
        Ok(crate::controllers::users_controller::all(&database)?)
    }

    field projects(
        &executor
    ) -> FieldResult<Vec<Project>> {
        let database = &executor.context().database;
        Ok(crate::controllers::projects_controller::all(&database)?)
    }

});

graphql_object!(MutationRoot: Context as "Mutation" |&self| {
    description: "The root mutation object of the schema"

    field login(
        &executor,
        email: String,
        password: String,
    ) -> FieldResult<LoginResponse>  {
        let database = &executor.context().database;
        let credentials = LoginRequest {
            email, password
        };
        let user = crate::controllers::users_controller::login(&credentials, &database)?;
        let token = crate::tokens::create_token(&user.email)?;
        Ok(LoginResponse {  user, token })
    }
});
