use crate::models::User;
use crate::schema::users;
use crate::types::*;
use diesel::prelude::*;
use rocket_contrib::databases::diesel;

pub fn all(conn: &diesel::PgConnection) -> Result<Vec<User>> {
    Ok(users::table.load::<User>(&*conn)?)
}

pub fn insert(user: InsertableUser, conn: &diesel::PgConnection) -> Result<Contributor> {
    Ok(diesel::insert_into(users::table)
        .values(&user)
        .get_result(conn)?)
}
