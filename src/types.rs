use crate::models::Project;
use crate::schema::projects;
use chrono::prelude::*;
use failure::Error;
use rocket_contrib::databases::diesel::Insertable;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

// Raw repository response from API
#[derive(Serialize, Deserialize, Debug)]
pub struct RepositoryResponse {
    pub created_at: String,
    pub updated_at: String,
    pub size: i64,
    pub forks_count: i64,
}

pub struct Repository {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub size: i64,
    pub forks_count: i64,
}

#[derive(Debug, Fail)]
pub enum AletheiaError {
    #[fail(display = "Config error: {}", message)]
    ConfigError { message: String },
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Debug)]
pub struct RulesConfig {
    pub start_date: String,
    pub end_date: String,
    pub max_collaborators: i64,
}

pub struct Rules {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub max_collaborators: i64,
}

#[database("postgres_logs")]
pub struct DbConn(diesel::PgConnection);

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "projects"]
pub struct InsertableProject {
    pub name: String,
    pub repository_url: String,
}

impl InsertableProject {
    pub fn from_project(project: Project) -> InsertableProject {
        InsertableProject {
            name: project.name,
            repository_url: project.repository_url,
        }
    }
}

#[derive(Debug)]
pub enum Issue {
    RepoCreationDate {
        start_date: DateTime<Utc>,
        repo_creation_date: DateTime<Utc>,
    },
    TeamSize {
        max_collaborators: i64,
        collaborators: i64,
    },
}

impl RulesConfig {
    pub fn into_rules(&self) -> Result<Rules> {
        Ok(Rules {
            start_date: self.start_date.parse::<DateTime<Utc>>()?,
            end_date: self.end_date.parse::<DateTime<Utc>>()?,
            max_collaborators: self.max_collaborators,
        })
    }
}

impl Repository {
    pub fn new(response: RepositoryResponse) -> Result<Repository> {
        Ok(Repository {
            created_at: response.created_at.parse::<DateTime<Utc>>()?,
            updated_at: response.updated_at.parse::<DateTime<Utc>>()?,
            size: response.size,
            forks_count: response.forks_count,
        })
    }
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Issue::RepoCreationDate {
                repo_creation_date,
                start_date,
            } => write!(
                f,
                "Repository created on {} but hackathon started {}",
                repo_creation_date.to_rfc2822(),
                start_date.to_rfc2822()
            ),
            Issue::TeamSize {
                collaborators,
                max_collaborators,
            } => write!(
                f,
                "Team size is {} but max team size is {}",
                collaborators, max_collaborators
            ),
        }
    }
}
