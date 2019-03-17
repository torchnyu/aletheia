#[derive(Debug, DbEnum)]
pub enum Verb {
    Create,
    Read,
    Update,
    Delete,
}

#[derive(Debug, DbEnum)]
pub enum Modifier {
    All,
    Self_,
}

pub struct Permission {
    id: i32,
    role_id: i32,
    resource_name: String,
    action: Verb,
    modifier: Modifier,
}

table! {
    use diesel::types::{Int4, Varchar};
    use diesel::sql_types::Nullable;
    use crate::types::{VerbMapping, ModifierMapping};
    permissions (id) {
        id -> Int4,
        role_id -> Int4,
        resource_name -> Nullable<Varchar>,
        action -> Nullable<VerbMapping>,
        modifier -> Nullable<ModifierMapping>,
    }
}
