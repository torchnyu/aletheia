use crate::db::schema::media;
use diesel::{self, AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "media"]
pub struct Medium {
    pub id: i32,
    pub folder_name: String,
    pub project_id: Option<i32>,
    pub user_id: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "media"]
pub struct MediumInsert {
    pub folder_name: String,
    pub project_id: Option<i32>,
    pub user_id: Option<i32>,
}
