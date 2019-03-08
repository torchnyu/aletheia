use crate::schema::*;
use diesel::{self, AsChangeset, Queryable};
use super::Context;
use serde_derive::{Deserialize, Serialize};
use crate::types::{Submission, UserResponse};
use crate::schema::users::dsl::{display_name, email, id};
use crate::schema::{submissions, users};
use diesel::pg::expression::dsl::any;
use diesel::BelongingToDsl;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;


#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "projects"]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub repository_url: String,
    pub color: String,
    pub description: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "projects"]
pub struct ProjectInsert {
    pub name: String,
    pub color: String,
    pub repository_url: String,
    pub description: Option<String>,
}

impl ProjectInsert {
    pub fn from_project(project: Project) -> ProjectInsert {
        ProjectInsert {
            name: project.name,
            color: project.color,
            repository_url: project.repository_url,
            description: project.description,
        }
    }
}

graphql_object!(Project: Context |&self| {
    description: "A hackathon project"

    field id(&executor) -> i32 {
        self.id
    }

    field name(&executor) -> &str {
        &self.name
    }

    field repository_url(&executor) -> &str {
        &self.repository_url
    }

    field color(&executor) -> &str {
        &self.color
    }

    field description(&executor) -> Option<&str> {
        match &self.description {
            Some(desc) => Some(desc.as_str()),
            None => None
        }
    }
    
    field contributors(&executor) -> Vec<UserResponse> {
        let database: &diesel::PgConnection = &executor.context().database;
        let user_ids = Submission::belonging_to(self).select(submissions::user_id);
        users::table.filter(users::id.eq(any(user_ids)))
            .select((id, display_name, email))
            .load::<UserResponse>(database).expect("Could not load contributors")
    }
});
