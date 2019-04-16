use serde_derive::{Deserialize, Serialize};

#[derive(Debug, DbEnum, Serialize, Deserialize)]
#[DieselType = "Action_type"]
pub enum ActionType {
    Create,
    Read,
    Update,
    Delete,
}

/// What kind of data are we acting on? i.e. all
/// data, our own data, some row?
#[derive(Debug, DbEnum, Serialize, Deserialize)]
#[DieselType = "Action_modifier"]
pub enum ActionModifier {
    All,
    Own,
    One,
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
    Event,
    #[db_rename = "user_event"]
    UserEvent,
}
