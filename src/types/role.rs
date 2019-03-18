use crate::schema::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "roles"]
pub struct Role {
    id: i32,
    name: String,
}
