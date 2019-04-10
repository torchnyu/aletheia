//use crate::db::schema::*;
use crate::db::schema::{users};
use diesel::{self, AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct RawUser {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub password_digest: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserInsert {
    pub display_name: Option<String>,
    pub email: String,
    pub password_digest: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub display_name: Option<String>,
    pub email: String,
    pub password: String,
}

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl User {
    pub fn from_raw_user(user: RawUser) -> User {
        User {
            id: user.id,
            display_name: user.display_name,
            email: user.email,
        }
    }
}
