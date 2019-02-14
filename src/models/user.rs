use crate::schema::*;
use crate::types::Result;
use argonautica::Hasher;
use diesel::{self, AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};

const SALT_LENGTH: u32 = 16;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub password_digest: String,
    pub salt: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserInsert {
    pub display_name: String,
    pub email: String,
    pub password_digest: String,
    pub salt: String,
}

impl UserInsert {
    pub fn from_request(request: UserRequest) -> Result<UserInsert> {
        let mut hasher = Hasher::default();
        let salt = "my salt!".to_string();
        let password_digest = hasher
            .with_password(request.password)
            .with_secret_key("secret key!")
            .with_salt(&salt)
            .hash()?;
        Ok(UserInsert {
            display_name: request.display_name,
            email: request.email,
            password_digest,
            salt,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub display_name: String,
    pub email: String,
    pub password: String,
}

// Don't send the password through the API idiot
#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserResult {
    pub id: i32,
    pub display_name: String,
    pub email: String,
}

impl UserResult {
    pub fn from_user(user: User) -> UserResult {
        UserResult {
            id: user.id,
            display_name: user.display_name,
            email: user.email,
        }
    }
}
