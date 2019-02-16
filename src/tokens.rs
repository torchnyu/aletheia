use crate::types::Result;
use chrono::{Duration, Local};
use jwt::{decode, encode, Header, Validation};
use serde_derive::{Deserialize, Serialize};
use std::convert::From;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    issuer: String,
    subject: String,
    issued_at: i64,
    expiry: i64,
    email: String,
}

static EXPIRY_DURATION: i64 = 72;

impl Claims {
    fn new(email: &str) -> Self {
        Claims {
            issuer: "localhost".into(),
            subject: "auth".into(),
            email: email.to_owned(),
            issued_at: Local::now().timestamp(),
            expiry: (Local::now() + Duration::hours(EXPIRY_DURATION)).timestamp(),
        }
    }
}

pub fn create_token(email: &str) -> Result<String> {
    let claims = Claims::new(email);
    let secret_key = env::var("SECRET_KEY")?;
    Ok(encode(&Header::default(), &claims, secret_key.as_bytes())?)
}
