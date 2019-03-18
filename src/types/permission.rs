use crate::schema::*;
use crate::sql_types::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "permissions"]
pub struct Permission {
    id: i32,
    role_id: i32,
    resource_name: String,
    action: ActionType,
    modifier: ActionModifier,
}
