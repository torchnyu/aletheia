#[derive(Debug, DbEnum)]
#[DieselType = "Action_type"]
pub enum ActionType {
    Create,
    Read,
    Update,
    Delete,
}

#[derive(Debug, DbEnum)]
#[DieselType = "Action_modifier"]
pub enum ActionModifier {
    All,
    #[db_rename = "self"]
    Self_,
}
