use crate::graphql::Context;
use crate::models::{Project, Submission, UserResponse};
use crate::schema::users::dsl::{display_name, email, id};
use crate::schema::{submissions, users};
use diesel::pg::expression::dsl::any;
use diesel::BelongingToDsl;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

graphql_object!(Project: Context |&self| {
    description: "A hackathon project"

    field id(&executor) -> i32 {
        self.id
    }

    field name(&executor) -> String {
        self.name.clone()
    }

    field repository_url(&executor) -> String {
        self.repository_url.clone()
    }
    
    field contributors(&executor) -> Vec<UserResponse> {
        let database: &diesel::PgConnection = &executor.context().database;
        let user_ids = Submission::belonging_to(self).select(submissions::user_id);
        users::table.filter(users::id.eq(any(user_ids)))
            .select((id, display_name, email))
            .load::<UserResponse>(database).expect("Could not load contributors")
    }
});
