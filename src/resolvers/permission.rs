use crate::db::schema::{permissions, user_roles};
use crate::db::sql_types::*;
use crate::db::orm::{Permission, User, UserRole};
use crate::utils::*;
use diesel::dsl::any;
use diesel::prelude::*;
use diesel::BelongingToDsl;
use rocket_contrib::databases::diesel;

pub fn get_permission(
    user: &User,
    resource: &str,
    action: &ActionType,
    modifier: &ActionModifier,
    conn: &diesel::PgConnection,
) -> Result<Vec<Permission>> {
    let role_ids = UserRole::belonging_to(user).select(user_roles::role_id);
    Ok(permissions::table
        .filter(permissions::role_id.eq(any(role_ids)))
        .filter(permissions::resource_name.eq(resource))
        .filter(permissions::action.contains(vec![action]))
        .filter(permissions::modifier.contains(vec![modifier]))
        .load::<Permission>(conn)?)
}
