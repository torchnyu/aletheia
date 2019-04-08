use crate::db::schema::submissions;
use crate::types::{Submission, SubmissionInsert};
use crate::utils::*;
use diesel::prelude::*;
use rocket_contrib::databases::diesel;

pub fn all(conn: &diesel::PgConnection) -> Result<Vec<Submission>> {
    Ok(submissions::table.load::<Submission>(&*conn)?)
}

pub fn get(id: i32, conn: &diesel::PgConnection) -> Result<Submission> {
    Ok(submissions::table.find(id).get_result::<Submission>(conn)?)
}

pub fn insert(project: SubmissionInsert, conn: &diesel::PgConnection) -> Result<Submission> {
    Ok(diesel::insert_into(submissions::table)
        .values(&project)
        .get_result(conn)?)
}

pub fn update(id: i32, person: Submission, conn: &diesel::PgConnection) -> Result<Submission> {
    Ok(diesel::update(submissions::table.find(id))
        .set(&person)
        .get_result(conn)?)
}

pub fn delete(id: i32, conn: &diesel::PgConnection) -> Result<usize> {
    Ok(diesel::delete(submissions::table.find(id)).execute(conn)?)
}
