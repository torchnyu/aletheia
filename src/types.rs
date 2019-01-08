use chrono::prelude::*;
use failure::Error;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    pub created_at: String,
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
    pub max_collaborators: i64,
}

pub struct Rules {
    pub start_date: DateTime<Utc>,
    pub max_collaborators: i64,
}

#[derive(Debug)]
pub enum Issues {
    Date {
        start_date: DateTime<Utc>,
        repo_date: DateTime<Utc>,
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
            max_collaborators: self.max_collaborators,
        })
    }
}
