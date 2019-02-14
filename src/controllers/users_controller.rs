use crate::models::{UserInsert, UserRequest, UserResult};
use crate::schema::users;
use crate::types::*;
use diesel::prelude::*;
use rocket_contrib::databases::diesel;

pub fn all(conn: &diesel::PgConnection) -> Result<Vec<UserResult>> {
    Ok(users::table
        .select((users::id, users::display_name, users::email))
        .load::<UserResult>(&*conn)?)
}

pub fn create(user: UserRequest, conn: &diesel::PgConnection) -> Result<UserResult> {
    let user = UserInsert::from_request(user)?;
    let user = diesel::insert_into(users::table)
        .values(&user)
        .get_result(conn)?;
    Ok(UserResult::from_user(user))
}
