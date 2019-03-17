use crate::sql_types::*;

pub struct Permission {
    id: i32,
    role_id: i32,
    resource_name: String,
    action: ActionType,
    modifier: ActionModifier,
}
