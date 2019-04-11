use crate::db::sql_types::{ActionModifier, ActionType};
use crate::resolvers::*;
use crate::types::Token;
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
    token: &Token,
    resource: String,
    action: ActionType,
    modifier: ActionModifier,
) -> Result<()> {
    let user = crate::resolvers::user::get_by_email(&token.uid, &conn)?;
    let permissions = permission::get_permission(&user, &resource, &action, &modifier, conn)?;
    if permissions.is_empty() {
        Err(AuthError::NoPermission { action, resource })?
    } else {
        Ok(())
    }
}
