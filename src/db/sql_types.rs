use serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, DbEnum, Serialize, Deserialize)]
#[DieselType = "Action_type"]
pub enum ActionType {
    Create,
    Read,
    Update,
    Delete,
}

/// What kind of data are we acting on? i.e. all
/// data, our own data, some row?
#[derive(Copy, Clone, Debug, DbEnum, Serialize, Deserialize)]
#[DieselType = "Action_modifier"]
pub enum ActionModifier {
    All,
    Own,
    One,
}

#[derive(Clone, Debug)]
pub enum Resource {
    Project,
    Submission,
    User,
    Permission,
    Role,
    UserRole,
    Event,
    UserEvent,
    Medium,
    None,
}

fn resource_to_string(resource: &Resource) -> String {
    match resource {
        Resource::Project => "project".to_string(),
        Resource::Submission => "submission".to_string(),
        Resource::User => "user".to_string(),
        Resource::Permission => "permissionr".to_string(),
        Resource::Role => "role".to_string(),
        Resource::UserRole => "user_role".to_string(),
        Resource::Event => "event".to_string(),
        Resource::UserEvent => "user_event".to_string(),
        Resource::Medium => "medium".to_string(),
        Resource::None => "none".to_string(),
    }
}

impl Into<String> for Resource {
    fn into(self) -> String {
        resource_to_string(&self)
    }
}

impl From<&Resource> for String {
    fn from(resource: &Resource) -> String {
        resource_to_string(resource)
    }
}
