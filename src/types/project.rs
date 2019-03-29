use super::Context;
use crate::resolvers;
use crate::schema::*;
use crate::types::{Event, UserResponse};
use diesel::{self, AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};
use slug::slugify;

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[belongs_to(Event)]
#[table_name = "projects"]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub repository_url: String,
    pub color: String,
    pub description: Option<String>,
    pub slug: String,
    pub event_id: i32,
}

#[derive(Insertable)]
#[table_name = "projects"]
pub struct ProjectInsert {
    pub name: String,
    pub repository_url: String,
    pub color: String,
    pub description: Option<String>,
    pub slug: String,
    pub event_id: i32,
}

#[derive(Serialize, Deserialize, GraphQLInputObject)]
pub struct ProjectRequest {
    pub name: String,
    pub repository_url: String,
    pub color: String,
    pub description: Option<String>,
    pub event_id: i32,
}

impl ProjectInsert {
    pub fn from_project(project: Project) -> ProjectInsert {
        ProjectInsert {
            name: project.name,
            color: project.color,
            repository_url: project.repository_url,
            description: project.description,
            slug: project.slug,
            event_id: project.event_id,
        }
    }
    pub fn from_request(request: ProjectRequest) -> ProjectInsert {
        let slug = slugify(&request.name);
        ProjectInsert {
            name: request.name,
            color: request.color,
            repository_url: request.repository_url,
            description: request.description,
            event_id: request.event_id,
            slug,
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

    field slug(&executor) -> &str {
        &self.slug
    }

    field description(&executor) -> Option<&str> {
        match &self.description {
            Some(desc) => Some(desc.as_str()),
            None => None
        }
    }

    field contributors(&executor) -> Vec<UserResponse> {
        let database: &diesel::PgConnection = &executor.context().database;
        resolvers::project::contributors(self, database)
    }
});
