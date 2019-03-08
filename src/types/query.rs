use super::{Context, Project, UserResponse};
use juniper::FieldResult;

pub struct QueryRoot {}

graphql_object!(QueryRoot: Context as "Query" |&self| {
    description: "The root query object of the schema"

    field users(
        &executor
    ) -> FieldResult<Vec<UserResponse>> {
        let database = &executor.context().database;
        Ok(crate::resolvers::user::all(&database)?)
    }

    field projects(
        &executor
    ) -> FieldResult<Vec<Project>> {
        let database = &executor.context().database;
        Ok(crate::resolvers::project::all(&database)?)
    }

});
