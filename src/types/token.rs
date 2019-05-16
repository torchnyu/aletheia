use crate::utils::Result;
use chrono::{Duration, Local};
use jwt::{decode, encode, Header, Validation};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    // Issuer
    pub iss: String,
    // Subject
    pub sub: String,
    // Issued at
    pub iua: i64,
    // Expiry
    pub exp: i64,
    // User email
    pub uid: String,
}

static EXPIRY_DURATION: i64 = 24;

#[derive(Debug, Fail)]
pub enum TokenError {
    #[fail(display = "Token expired")]
    Expired,
    #[fail(display = "Could not parse token")]
    ParseFailure,
    #[fail(display = "Wrong number of tokens: {}", num)]
    ArityMismatch { num: usize },
}

impl Token {
    pub fn new(email: &str) -> Self {
        Token {
            iss: "localhost".into(),
            sub: "auth".into(),
            uid: email.to_owned(),
            iua: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(EXPIRY_DURATION)).timestamp(),
        }
    }

    pub fn new_invalid() -> Self {
        let time = Local::now() - Duration::hours(EXPIRY_DURATION);
        Token {
            iss: "".into(),
            sub: "noauth".into(),
            uid: "".into(),
            iua: (time - Duration::hours(EXPIRY_DURATION)).timestamp(),
            exp: time.timestamp(),
        }
    }

    pub fn validate(&self) -> Result<Self> {
        let is_expired = Local::now().timestamp() > self.exp;
        if is_expired {
            return Err(TokenError::Expired)?;
        }
        Ok(Self::new(&self.uid))
    }

    pub fn to_string(&self) -> Result<String> {
        let secret_key = env::var("SECRET_KEY")?;
        Ok(encode(&Header::default(), &self, secret_key.as_bytes())?)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = TokenError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Token, Self::Error> {
        let keys: Vec<_> = request.headers().get("token").collect();
        if keys.len() != 1 {
            return Outcome::Failure((
                Status::BadRequest,
                TokenError::ArityMismatch { num: keys.len() },
            ));
        }

        if let Ok(token) = keys[0].parse::<Token>() {
            match token.validate() {
                Err(_err) => Outcome::Failure((Status::BadRequest, TokenError::Expired)),
                Ok(new_token) => Outcome::Success(new_token),
            }
        } else {
            Outcome::Failure((Status::BadRequest, TokenError::ParseFailure))
        }
    }
}

pub fn create_token(email: &str) -> Result<String> {
    let claims = Token::new(email);
    let secret_key = env::var("SECRET_KEY")?;
    Ok(encode(&Header::default(), &claims, secret_key.as_bytes())?)
}

impl FromStr for Token {
    type Err = failure::Error;

    fn from_str(string: &str) -> Result<Self> {
        let secret_key = env::var("SECRET_KEY")?;
        let token = decode::<Token>(&string, secret_key.as_bytes(), &Validation::default())?;
        Ok(token.claims)
    }
}
