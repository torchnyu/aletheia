use crate::db::schema::{permissions};
use crate::db::sql_types::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations, Debug)]
#[table_name = "permissions"]
pub struct Permission {
    id: i32,
    role_id: i32,
    action: Vec<ActionType>,
    modifier: Vec<ActionModifier>,
    resource_name: String,
}
