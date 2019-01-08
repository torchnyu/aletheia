use crate::errors::{AletheiaError, Result};
use chrono::prelude::*;
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Repository {
    created_at: String,
}

pub fn check_repos(repos: &[&'static str], start_date: i64) -> Result<()> {
    let client = Client::new();
    for repo in repos {
        let url = format!("https://api.github.com/repos/{}", repo);
        let body = client.get(&url).send()?.text()?;
        let repo: Repository = serde_json::from_str(&body)?;
        let created_at_date = repo.created_at.parse::<DateTime<Utc>>()?;
        if start_date > created_at_date.timestamp() {
            return Err(AletheiaError::DateError {
                date: created_at_date.to_string(),
            })?;
        }
    }
    Ok(())
}
