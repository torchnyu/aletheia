use crate::db::schema::password_reset_requests;
use chrono::NaiveDateTime;
use diesel::{self, AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};

#[derive(Identifiable, Insertable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "password_reset_requests"]
pub struct PasswordResetRequest {
    pub id: String,
    pub created_at: Option<NaiveDateTime>,
    pub user_id: i32,
}
