use crate::utils::Result;
use chrono::{Duration, Local};
use jwt::{decode, encode, Header, Validation};
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // Issuer
    iss: String,
    // Subject
    sub: String,
    // Issued at
    iua: i64,
    // Expiry
    exp: i64,
    // User email
    uid: String,
}

static EXPIRY_DURATION: i64 = 24;

#[derive(Debug, Fail)]
pub enum TokenError {
    #[fail(display = "Token expired")]
    Expired,
}

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

    pub fn validate(&self) -> Result<Self> {
        let is_not_expired = Local::now().timestamp() < self.exp;
        if is_not_expired {
            Ok(Self::new(&self.uid))
        } else {
            Err(TokenError::Expired)?
        }
    }

    pub fn to_string(&self) -> Result<String> {
        let secret_key = env::var("SECRET_KEY")?;
        Ok(encode(&Header::default(), &self, secret_key.as_bytes())?)
    }
}

pub fn create_token(email: &str) -> Result<String> {
    let claims = Claims::new(email);
    let secret_key = env::var("SECRET_KEY")?;
    Ok(encode(&Header::default(), &claims, secret_key.as_bytes())?)
}

impl FromStr for Claims {
    type Err = failure::Error;

    fn from_str(string: &str) -> Result<Self> {
        let secret_key = env::var("SECRET_KEY")?;
        let token = decode::<Claims>(&string, secret_key.as_bytes(), &Validation::default())?;
        Ok(token.claims)
    }
}
