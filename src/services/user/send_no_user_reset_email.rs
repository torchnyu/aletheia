use crate::utils::Result;
use std::env;

pub fn call() -> Result<()> {
    let client = reqwest::Client::new();
    let hermes_url = env::var("HERMES_URL")?;
    client
        .post(&format!("{}/no-user-reset", hermes_url))
        .send()?;
    Ok(())
}
