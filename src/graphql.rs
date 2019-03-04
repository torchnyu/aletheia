use crate::db::Connection;
use crate::models::UserResponse;
use crate::types::Result;
use juniper::FieldResult;
use juniper::RootNode;

pub struct Context {
    pub database: Connection,
}

type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    RootNode::new(QueryRoot {}, MutationRoot {})
}

pub struct QueryRoot {}
pub struct MutationRoot {}

graphql_object!(QueryRoot: Context as "Query" |&self| {
    description: "The root query object of the schema"

    field get_all_users(
        &executor
    ) -> FieldResult<Vec<UserResponse>> {
        let database = &executor.context().database;
        Ok(crate::controllers::users_controller::all(&database)?)
    }

});

graphql_object!(MutationRoot: Context as "Mutation" |&self| {
    description: "The root mutation object of the schema"

    field create_talk(
        &executor,
        test_arg: String as "Some argumetn"
    ) -> String as "Does something" {
        "Blah".to_string()
    }
});
