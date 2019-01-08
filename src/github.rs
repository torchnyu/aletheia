use crate::types::{Issue, Repository, Result, Rules};
use reqwest::Client;

static GITHUB_API: &'static str = "https://api.github.com/repos/";

pub fn check_repos(repos: &[&'static str], rules: Rules) -> Result<Vec<Issue>> {
    let client = Client::new();
    let mut issues = Vec::new();
    for repo in repos {
        let url = format!("{}{}", GITHUB_API, repo);
        let body = client.get(&url).send()?.text()?;
        let repo: Repository = Repository::new(serde_json::from_str(&body)?)?;
        if rules.start_date.timestamp() > repo.created_at.timestamp() {
            issues.push(Issue::RepoCreationDate {
                start_date: rules.start_date,
                repo_creation_date: repo.created_at,
            });
        }
        if repo.forks_count > rules.max_collaborators {
            issues.push(Issue::TeamSize {
                max_collaborators: rules.max_collaborators,
                collaborators: repo.forks_count,
            });
        }
    }
    Ok(issues)
}
