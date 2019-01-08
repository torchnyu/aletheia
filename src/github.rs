use crate::errors::Result;
use chrono::prelude::*;
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Repository {
    created_at: String,
}

pub fn check_repos(repos: &[&'static str]) -> Result<()> {
    let client = Client::new();
    for repo in repos {
        let url = format!("https://api.github.com/repos/{}", repo);
        let body = client.get(&url).send()?.text()?;
        let repo: Repository = serde_json::from_str(&body)?;
        println!("{:?}", repo);
    }
    Ok(())
}
