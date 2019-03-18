use crate::resolvers::*;
use crate::sql_types::{ActionModifier, ActionType};
use crate::types::UserResponse;
use crate::utils::Result;

#[derive(Debug, Fail)]
pub enum AuthError {
    #[fail(display = "You are not authorized to {:?} {}", action, resource)]
    NoPermission {
        action: ActionType,
        resource: String,
    },
}

pub fn validate(
    conn: &diesel::PgConnection,
    user: &UserResponse,
    resource: String,
    action: ActionType,
    modifier: ActionModifier,
) -> Result<()> {
    let permissions = permission::get_permission(&user, &resource, &action, &modifier, conn)?;
    if permissions.is_empty() {
        Err(AuthError::NoPermission { action, resource })?
    } else {
        Ok(())
    }
}
