use crate::db::models::{PasswordResetRequest, User};
use crate::utils::Result;
use argonautica::Hasher;
use rand::Rng;
use std::env;

static RESET_KEY_LENGTH: usize = 16;

pub fn call(user: &User) -> Result<PasswordResetRequest> {
    let rand_bytes = rand::thread_rng().gen_ascii_chars().take(16).collect();
    let hasher = Hasher::default();
    // Hash the random bytes
    let id = hasher
        .with_password(rand_bytes)
        .with_secret_key(env::var("SECRET_KEY")?)
        .hash()?;
    Ok(PasswordResetRequest {
        id,
        created_at: None,
        user_id: user.id,
    })
}
