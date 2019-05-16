use crate::db::connection::DatabaseContext;
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

pub fn validate(db: &DatabaseContext, resource: String) -> Result<()> {
    let user = crate::resolvers::user::get_by_email(&db.token.uid, db)?;
    let permissions = permission::get_permission(&user, &resource, &db.action, &db.modifier, db)?;
    if permissions.is_empty() {
        Err(AuthError::NoPermission {
            action: db.action,
            resource,
        })?
    } else {
        Ok(())
    }
}
