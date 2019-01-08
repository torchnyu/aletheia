use crate::errors::Result;
use chrono::prelude::*;
use reqwest::Client;

pub fn check_repos(repos: &[&'static str]) -> Result<()> {
    let client = Client::new();
    for repo in repos {
        let url = format!("https://api.github.com/repos/{}", repo);
        let body = client.get(&url).send()?.text()?;
        let parsed_body: serde_json::Value = serde_json::from_str(&body)?;
        let created_str = (&parsed_body["created_at"]).as_str();
        if let Some(s) = created_str {
            let creation_date = s.parse::<DateTime<Utc>>();
            println!("{:?}", creation_date);
        }
    }
    Ok(())
}
