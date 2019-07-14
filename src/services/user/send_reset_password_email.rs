use crate::db::models::{PasswordResetRequest, User};
use crate::utils::Result;
use argonautica::Hasher;
use chrono::Utc;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::collections::HashMap;
use std::env;

static RESET_KEY_LENGTH: usize = 16;

pub fn call(user: &User, domain: &str) -> Result<PasswordResetRequest> {
    let rand_bytes: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(RESET_KEY_LENGTH)
        .collect();
    send_email(user, &rand_bytes, domain)?;
    let mut hasher = Hasher::default();
    // Hash the random bytes
    let id = hasher
        .with_password(rand_bytes)
        .with_salt(env::var("KEY_SALT")?)
        .with_secret_key(env::var("SECRET_KEY")?)
        .hash()?;
    Ok(PasswordResetRequest {
        id,
        created_at: Utc::now(),
        user_id: user.id,
    })
}

fn send_email(user: &User, key: &str, domain: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let mut body = HashMap::new();
    body.insert("email", user.email.as_str());
    body.insert("resetKey", key);
    body.insert("domain", domain);
    let hermes_url = env::var("HERMES_URL")?;
    client
        .post(&format!("{}/reset-password", hermes_url))
        .json(&body)
        .send()?;
    Ok(())
}
