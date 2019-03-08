use crate::schema::*;
use diesel::{self, AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "projects"]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub repository_url: String,
    pub color: String,
    pub description: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "projects"]
pub struct ProjectInsert {
    pub name: String,
    pub color: String,
    pub repository_url: String,
    pub description: String,
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
