use super::Context;
use crate::schema::*;
use crate::schema::{projects, submissions};
use crate::types::{Project, Submission};
use diesel::pg::expression::dsl::any;
use diesel::BelongingToDsl;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::{self, AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct RawUser {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub password_digest: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserInsert {
    pub display_name: Option<String>,
    pub email: String,
    pub password_digest: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub display_name: Option<String>,
    pub email: String,
    pub password: String,
}

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub email: String,
}

graphql_object!(User: Context |&self| {
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

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl User {
    pub fn from_raw_user(user: RawUser) -> User {
        User {
            id: user.id,
            display_name: user.display_name,
            email: user.email,
        }
    }
}
