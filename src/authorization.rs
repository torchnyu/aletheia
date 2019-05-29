use crate::db::connection::DatabaseContext;
use crate::db::sql_types::ActionType;
use crate::resolvers::*;
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
    let user = db.user.as_ref().ok_or(AuthError::NoPermission {
        action: db.action,
        resource: resource.clone(),
    })?;

    let permissions = permission::get_permission(user, &resource, db)?;
    if permissions.is_empty() {
        Err(AuthError::NoPermission {
            action: db.action,
            resource,
        })?
    } else {
        Ok(())
    }
}
