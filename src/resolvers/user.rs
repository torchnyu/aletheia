use crate::schema::users;
use crate::types::{LoginRequest, User, UserInsert, UserRequest, UserResponse};
use crate::utils::{AletheiaError, Result};
use diesel::dsl::*;
use diesel::prelude::*;
use rocket_contrib::databases::diesel;

pub fn all(conn: &diesel::PgConnection) -> Result<Vec<UserResponse>> {
    Ok(users::table
        .select((users::id, users::display_name, users::email))
        .load::<UserResponse>(&*conn)?)
}

pub fn create(user: UserRequest, conn: &diesel::PgConnection) -> Result<UserResponse> {
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
    Ok(UserResponse::from_user(user))
}

pub fn login(credentials: &LoginRequest, conn: &diesel::PgConnection) -> Result<UserResponse> {
    let user: User = users::table
        .filter(users::email.eq(&(credentials.email)))
        .first(conn)?;
    if user.validate_credentials(credentials)? {
        Ok(UserResponse::from_user(user))
    } else {
        Err(AletheiaError::NoUserError {
            email: credentials.email.clone(),
        })?
    }
}

pub fn get_by_email(email: &str, conn: &diesel::PgConnection) -> Result<UserResponse> {
    let user: User = users::table.filter(users::email.eq(email)).first(conn)?;
    Ok(UserResponse::from_user(user))
}
