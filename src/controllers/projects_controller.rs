use crate::schema::projects;
use crate::types::{Project, ProjectInsert};
use crate::utils::*;
use diesel::prelude::*;
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
