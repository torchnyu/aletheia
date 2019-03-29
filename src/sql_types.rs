use serde_derive::{Deserialize, Serialize};

#[derive(Debug, DbEnum, Serialize, Deserialize)]
#[DieselType = "Action_type"]
pub enum ActionType {
    Create,
    Read,
    Update,
    Delete,
}

#[derive(Debug, DbEnum, Serialize, Deserialize)]
#[DieselType = "Action_modifier"]
pub enum ActionModifier {
    All,
    Own,
}

#[derive(Debug, DbEnum, Serialize, Deserialize)]
#[DieselType = "Resource"]
pub enum Type {
    Project,
    Submission,
    User,
    Permission,
    Role,
    #[db_rename = "user_role"]
    UserRole,
}
