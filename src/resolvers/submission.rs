use crate::db::connection::DatabaseContext;
use crate::db::models::{Submission, SubmissionInsert};
use crate::db::schema::submissions;
use crate::utils::*;
use diesel::prelude::*;
use rocket_contrib::databases::diesel;

pub fn all(db: &DatabaseContext) -> Result<Vec<Submission>> {
    Ok(submissions::table.load::<Submission>(db.conn)?)
}

pub fn get(id: i32, db: &DatabaseContext) -> Result<Submission> {
    Ok(submissions::table
        .find(id)
        .get_result::<Submission>(db.conn)?)
}

pub fn insert(project: SubmissionInsert, db: &DatabaseContext) -> Result<Submission> {
    Ok(diesel::insert_into(submissions::table)
        .values(&project)
        .get_result(db.conn)?)
}

pub fn update(id: i32, person: Submission, db: &DatabaseContext) -> Result<Submission> {
    Ok(diesel::update(submissions::table.find(id))
        .set(&person)
        .get_result(db.conn)?)
}

pub fn delete(id: i32, db: &DatabaseContext) -> Result<usize> {
    Ok(diesel::delete(submissions::table.find(id)).execute(db.conn)?)
}
