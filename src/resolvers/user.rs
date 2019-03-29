use crate::schema::{roles, user_roles, users};
use crate::types::{LoginRequest, RawUser, Role, User, UserInsert, UserRequest, UserRole};
use crate::utils::{AletheiaError, Result};
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::BelongingToDsl;
use rocket_contrib::databases::diesel;

pub fn all(conn: &diesel::PgConnection) -> Result<Vec<User>> {
    Ok(users::table
        .select((users::id, users::display_name, users::email))
        .load::<User>(&*conn)?)
}

pub fn create(user: UserRequest, conn: &diesel::PgConnection) -> Result<User> {
    let user_exists = select(exists(
        users::table
            .filter(users::email.eq(&(user.email)))
            .or_filter(users::display_name.eq(&(user.display_name))),
    ))
    .get_result(conn)?;
    if user_exists {
        return Err(AletheiaError::UserAlreadyExists {
            email: user.email.clone(),
        })?;
    }
    let user = UserInsert::from_request(user)?;
    let user = diesel::insert_into(users::table)
        .values(&user)
        .get_result(conn)?;
    Ok(User::from_raw_user(user))
}

pub fn login(credentials: &LoginRequest, conn: &diesel::PgConnection) -> Result<User> {
    let user: RawUser = users::table
        .filter(users::email.eq(&(credentials.email)))
        .first(conn)?;
    if user.validate_credentials(credentials)? {
        Ok(User::from_raw_user(user))
    } else {
        Err(AletheiaError::NoUserError {
            email: credentials.email.clone(),
        })?
    }
}

pub fn get_by_email(email: &str, conn: &diesel::PgConnection) -> Result<User> {
    let user: RawUser = users::table.filter(users::email.eq(email)).first(conn)?;
    Ok(User::from_raw_user(user))
}

impl User {
    pub fn roles(&self, conn: &diesel::PgConnection) -> Vec<Role> {
        let role_ids = UserRole::belonging_to(self).select(user_roles::role_id);
        roles::table
            .filter(roles::id.eq(any(role_ids)))
            .load::<Role>(conn)
            .expect("Could not load contributors")
    }
}
