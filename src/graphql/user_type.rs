use super::Context;
use crate::models::{Submission, Project};
use crate::schema::{projects, submissions};
use diesel::pg::expression::dsl::any;
use diesel::BelongingToDsl;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub use crate::models::UserResponse;


graphql_object!(UserResponse: Context as "User" |&self| {
    description: "A user"

    field id(&executor) -> i32 {
        self.id
    }

    field display_name(&executor) -> &str {
        &self.display_name
    }

    field email(&executor) -> &str {
        &self.email
    }
    
    field projects(&executor) -> Vec<Project> {
        let database: &diesel::PgConnection = &executor.context().database;
        let project_ids = Submission::belonging_to(self).select(submissions::project_id);
        projects::table
            .filter(projects::id.eq(any(project_ids)))
            .load::<Project>(database).expect("Could not load projects")
    }
});
