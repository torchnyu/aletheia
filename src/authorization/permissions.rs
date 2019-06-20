use crate::db::models::{Permission, User, UserRole};
use crate::db::schema::{permissions, user_roles};
use crate::db::sql_types::*;
use crate::utils::*;
use diesel::dsl::any;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::BelongingToDsl;
use rocket_contrib::databases::diesel;

pub fn get_permission(
    conn: &PgConnection,
    user: &Option<User>,
    resource: &Resource,
    action: &ActionType,
    modifier: &ActionModifier,
) -> Result<Vec<Permission>> {
    let role_ids = if let Some(user) = user {
        UserRole::belonging_to(user).select(user_roles::role_id)
    } else {
        roles::table.filter(roles::name.eq("guest".to_string()))
    };
    let resource_name: String = resource.into();
    Ok(permissions::table
        .filter(permissions::role_id.eq(any(role_ids)))
        .filter(permissions::resource_name.eq(resource_name))
        .filter(permissions::action.contains(vec![&action]))
        .filter(permissions::modifier.contains(vec![&modifier]))
        .load::<Permission>(conn)?)
}
