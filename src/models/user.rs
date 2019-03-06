use crate::schema::*;
use crate::types::Result;
use argonautica::input::Salt;
use argonautica::{Hasher, Verifier};
use diesel::{self, AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};
use std::env;

const SALT_LENGTH: u32 = 16;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub password_digest: String,
}

impl User {
    pub fn validate_credentials(self: &User, creds: &LoginRequest) -> Result<bool> {
        let mut verifier = Verifier::default();

        Ok(verifier
            .with_hash(self.password_digest.clone())
            .with_password(creds.password.clone())
            .with_secret_key(env::var("SECRET_KEY")?)
            .verify()?)
    }
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserInsert {
    pub display_name: String,
    pub email: String,
    pub password_digest: String,
}

impl UserInsert {
    pub fn from_request(request: UserRequest) -> Result<UserInsert> {
        let mut hasher = Hasher::default();
        let salt = Salt::random(SALT_LENGTH);
        let salt = salt.to_str()?;
        let password_digest = hasher
            .with_password(request.password)
            .with_secret_key(env::var("SECRET_KEY")?)
            .with_salt(salt)
            .hash()?;
        Ok(UserInsert {
            display_name: request.display_name,
            email: request.email,
            password_digest,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "users"]
pub struct UserResponse {
    pub id: i32,
    pub display_name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(GraphQLObject)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

impl UserResponse {
    pub fn from_user(user: User) -> UserResponse {
        UserResponse {
            id: user.id,
            display_name: user.display_name,
            email: user.email,
        }
    }
}
