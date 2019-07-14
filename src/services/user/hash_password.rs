use crate::utils::Result;
use argonautica::input::Salt;
use argonautica::Hasher;
use std::env;

pub fn call(password: &str) -> Result<String> {
    let mut hasher = Hasher::default();
    let salt_length = env::var("SALT_LENGTH")?;
    let salt = Salt::random(salt_length.parse::<u32>()?);
    let salt = salt.to_str()?;
    Ok(hasher
        .with_password(password)
        .with_secret_key(env::var("SECRET_KEY")?)
        .with_salt(salt)
        .hash()?)
}
