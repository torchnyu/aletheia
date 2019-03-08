use super::{Context, LoginRequest, LoginResponse, Project, UserResponse};
use crate::db::Connection;
use juniper::Context as JuniperContext;
use juniper::FieldResult;

pub struct QueryRoot {}

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
