use crate::db::connection::DatabaseContext;
use crate::db::models::{Permission, User, UserRole};
use crate::db::schema::{permissions, user_roles};
use crate::utils::*;
use diesel::dsl::any;
use diesel::prelude::*;
use diesel::BelongingToDsl;
use rocket_contrib::databases::diesel;

pub fn get_permission(
    user: &User,
    resource: &str,
    db: &DatabaseContext,
) -> Result<Vec<Permission>> {
    let action = &db.action;
    let modifier = &db.modifier;
    let role_ids = UserRole::belonging_to(user).select(user_roles::role_id);
    Ok(permissions::table
        .filter(permissions::role_id.eq(any(role_ids)))
        .filter(permissions::resource_name.eq(resource))
        .filter(permissions::action.contains(vec![action]))
        .filter(permissions::modifier.contains(vec![modifier]))
        .load::<Permission>(db.conn)?)
}
