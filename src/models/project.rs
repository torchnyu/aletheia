use crate::schema::*;
use diesel::{self, AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "projects"]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub repository_url: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "projects"]
pub struct ProjectInsert {
    pub name: String,
    pub repository_url: String,
}

impl ProjectInsert {
    pub fn from_project(project: Project) -> ProjectInsert {
        ProjectInsert {
            name: project.name,
            repository_url: project.repository_url,
        }
    }
}
