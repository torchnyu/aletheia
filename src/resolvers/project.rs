use crate::schema::users::columns;
use crate::schema::{projects, submissions, users};
use crate::types::{Project, ProjectInsert, Submission, UserResponse};
use crate::utils::*;
use diesel::dsl::any;
use diesel::prelude::*;
use diesel::BelongingToDsl;
use rocket_contrib::databases::diesel;

pub fn all(conn: &diesel::PgConnection) -> Result<Vec<Project>> {
    Ok(projects::table.load::<Project>(&*conn)?)
}

pub fn get(id: i32, conn: &diesel::PgConnection) -> Result<Project> {
    Ok(projects::table.find(id).get_result::<Project>(conn)?)
}

pub fn insert(project: ProjectInsert, conn: &diesel::PgConnection) -> Result<Project> {
    Ok(diesel::insert_into(projects::table)
        .values(&project)
        .get_result(conn)?)
}

pub fn update(id: i32, person: Project, conn: &diesel::PgConnection) -> Result<Project> {
    Ok(diesel::update(projects::table.find(id))
        .set(&person)
        .get_result(conn)?)
}

pub fn delete(id: i32, conn: &diesel::PgConnection) -> Result<usize> {
    Ok(diesel::delete(projects::table.find(id)).execute(conn)?)
}

pub fn contributors(project: &Project, conn: &diesel::PgConnection) -> Vec<UserResponse> {
    let user_ids = Submission::belonging_to(project).select(submissions::user_id);
    users::table
        .filter(users::id.eq(any(user_ids)))
        .select((columns::id, columns::display_name, columns::email))
        .load::<UserResponse>(conn)
        .expect("Could not load contributors")
}
