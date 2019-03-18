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
    #[db_rename = "self"]
    Self_,
}
