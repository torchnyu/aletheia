use crate::schema::*;
use diesel::{self, AsChangeset, Queryable};
use super::Context;
use serde_derive::{Deserialize, Serialize};
use crate::types::{UserResponse};
use crate::resolvers;
use slug::slugify;


#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "projects"]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub repository_url: String,
    pub color: String,
    pub description: Option<String>,
    pub slug: String
}

#[derive(Insertable)]
#[table_name = "projects"]
pub struct ProjectInsert {
    pub name: String,
    pub repository_url: String,
    pub color: String,
    pub description: Option<String>,
    pub slug: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectRequest {
    pub name: String,
    pub repository_url: String,
    pub color: String,
    pub description: Option<String>,    
}

impl ProjectInsert {
    pub fn from_project(project: Project) -> ProjectInsert {
        ProjectInsert {
            name: project.name,
            color: project.color,
            repository_url: project.repository_url,
            description: project.description,
            slug: project.slug
        }
    }
    pub fn from_request(request: ProjectRequest) -> ProjectInsert {
        let slug = slugify(&request.name);
        ProjectInsert {
            name: request.name,
            color: request.color,
            repository_url: request.repository_url,
            description: request.description,
            slug
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

#[derive(Serialize, Deserialize)]
pub struct Tokenized<T> {
    pub inner: T,
    pub token: String
}

graphql_object!(Tokenized<Project>: Context as "AuthenticatedProject" |&self| {
    field token() -> &str {
        &self.token
    }

    field project() -> &Project {
        &self.inner
    }
});
