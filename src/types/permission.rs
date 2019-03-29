use crate::schema::*;
use crate::sql_types::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations, Debug)]
#[table_name = "permissions"]
pub struct Permission {
    id: i32,
    role_id: i32,
    resource_name: Type,
    action: Vec<ActionType>,
    modifier: Vec<ActionModifier>,
}
