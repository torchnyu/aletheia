use crate::db::sql_types::{ActionModifier, ActionType, Resource};
use crate::types::Token;
use crate::utils::Result;

#[derive(Debug, Fail)]
pub enum AuthError {
    #[fail(display = "You are not authorized to {:?} {:?}", action, resource)]
    NoPermission {
        action: ActionType,
        resource: Resource,
    },
    #[fail(display = "Key has expired, please request another one")]
    ExpiredResetKey,
}

pub fn validate(
    conn: &diesel::PgConnection,
    token: &Token,
    resource: Resource,
    action: ActionType,
    modifier: ActionModifier,
) -> Result<()> {
    let user = crate::resolvers::user::get_by_email(&token.uid, conn)?;
    let permissions =
        crate::resolvers::permission::get_permission(conn, &user, &resource, &action, &modifier)?;
    if permissions.is_empty() {
        Err(AuthError::NoPermission { action, resource })?
    } else {
        Ok(())
    }
}
