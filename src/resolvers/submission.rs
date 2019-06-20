use crate::db::models::{Submission, SubmissionInsert};
use crate::db::schema::submissions;
use crate::db::PgConnection;
use crate::utils::*;
use diesel::prelude::*;
use rocket_contrib::databases::diesel;

pub fn all(db: &PgConnection) -> Result<Vec<Submission>> {
    Ok(submissions::table.load::<Submission>(db)?)
}

pub fn get(id: i32, db: &PgConnection) -> Result<Submission> {
    Ok(submissions::table.find(id).get_result::<Submission>(db)?)
}

pub fn insert(project: SubmissionInsert, db: &PgConnection) -> Result<Submission> {
    Ok(diesel::insert_into(submissions::table)
        .values(&project)
        .get_result(db)?)
}

pub fn update(id: i32, person: Submission, db: &PgConnection) -> Result<Submission> {
    Ok(diesel::update(submissions::table.find(id))
        .set(&person)
        .get_result(db)?)
}

pub fn delete(id: i32, db: &PgConnection) -> Result<usize> {
    Ok(diesel::delete(submissions::table.find(id)).execute(db)?)
}
