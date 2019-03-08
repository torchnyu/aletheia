use crate::utils::Result;
use chrono::{Duration, Local};
use jwt::{decode, encode, Header, Validation};
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    iua: i64,
    exp: i64,
    uid: String,
}

static EXPIRY_DURATION: i64 = 72;

impl Claims {
    fn new(email: &str) -> Self {
        Claims {
            iss: "localhost".into(),
            sub: "auth".into(),
            uid: email.to_owned(),
            iua: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(EXPIRY_DURATION)).timestamp(),
        }
    }
}

pub fn create_token(email: &str) -> Result<String> {
    let claims = Claims::new(email);
    let secret_key = env::var("SECRET_KEY")?;
    Ok(encode(&Header::default(), &claims, secret_key.as_bytes())?)
}
