use crate::models::Contributor;
use crate::schema::contributors;
use crate::types::*;
use diesel::prelude::*;
use rocket_contrib::databases::diesel;

pub fn all(conn: &diesel::PgConnection) -> Result<Vec<Contributor>> {
    Ok(contributors::table.load::<Contributor>(&*conn)?)
}

pub fn insert(
    contributor: InsertableContributor,
    conn: &diesel::PgConnection,
) -> Result<Contributor> {
    Ok(diesel::insert_into(contributors::table)
        .values(&contributor)
        .get_result(conn)?)
}
