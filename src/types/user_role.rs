use super::{Role, User, UserResponse};
use crate::schema::*;
use diesel::{self, AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "user_roles"]
#[belongs_to(User)]
#[belongs_to(UserResponse, foreign_key = "user_id")]
#[belongs_to(Role)]
pub struct UserRole {
    id: i32,
    user_id: i32,
    role_id: i32,
}
