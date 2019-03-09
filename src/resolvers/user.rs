use crate::schema::users::dsl::{display_name, email, id, users};
use crate::types::{LoginRequest, User, UserInsert, UserRequest, UserResponse};
use crate::utils::{AletheiaError, Result};
use diesel::dsl::*;
use diesel::prelude::*;
use rocket_contrib::databases::diesel;

pub fn all(conn: &diesel::PgConnection) -> Result<Vec<UserResponse>> {
    Ok(users
        .select((id, display_name, email))
        .load::<UserResponse>(&*conn)?)
}

pub fn create(user: UserRequest, conn: &diesel::PgConnection) -> Result<UserResponse> {
    let user_exists = select(exists(users.filter(email.eq(&(user.email))))).get_result(conn)?;
    if user_exists {
        return Err(AletheiaError::UserAlreadyExists {
            email: user.email.clone(),
        })?;
    }
    let user = UserInsert::from_request(user)?;
    let user = diesel::insert_into(users).values(&user).get_result(conn)?;
    Ok(UserResponse::from_user(user))
}

pub fn login(credentials: &LoginRequest, conn: &diesel::PgConnection) -> Result<UserResponse> {
    let user: User = users.filter(email.eq(&(credentials.email))).first(conn)?;
    if user.validate_credentials(credentials)? {
        Ok(UserResponse::from_user(user))
    } else {
        Err(AletheiaError::NoUserError {
            email: credentials.email.clone(),
        })?
    }
}
