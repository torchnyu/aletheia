use crate::types::{AletheiaError, Issues, Repository, Result, Rules};
use chrono::prelude::*;
use reqwest::Client;

pub fn check_repos(repos: &[&'static str], rules: Rules) -> Result<Vec<Issues>> {
    let client = Client::new();
    let mut issues = Vec::new();
    for repo in repos {
        let url = format!("https://api.github.com/repos/{}", repo);
        let body = client.get(&url).send()?.text()?;
        let repo: Repository = serde_json::from_str(&body)?;
        let created_at_date = repo.created_at.parse::<DateTime<Utc>>()?;
        if rules.start_date.timestamp() > created_at_date.timestamp() {
            issues.push(Issues::Date {
                start_date: rules.start_date,
                repo_date: created_at_date,
            });
        }
        if repo.forks_count > rules.max_collaborators {
            issues.push(Issues::TeamSize {
                max_collaborators: rules.max_collaborators,
                collaborators: repo.forks_count,
            });
        }
    }
    Ok(issues)
}
