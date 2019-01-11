use crate::models::Project;
use crate::schema::projects;
use crate::types::*;
use diesel;
use diesel::prelude::*;

pub fn all(connection: &PgConnection) -> Result<Vec<Project>> {
    Ok(projects::table.load::<Project>(&*connection)?)
}

pub fn get(id: i32, connection: &PgConnection) -> Result<Project> {
    Ok(projects::table.find(id).get_result::<Project>(connection)?)
}

pub fn insert(project: InsertableProject, connection: &PgConnection) -> Result<Project> {
    Ok(diesel::insert_into(projects::table)
        .values(&project)
        .get_result(connection)?)
}

pub fn update(id: i32, person: Project, connection: &PgConnection) -> Result<Project> {
    Ok(diesel::update(projects::table.find(id))
        .set(&person)
        .get_result(connection)?)
}

pub fn delete(id: i32, connection: &PgConnection) -> Result<usize> {
    Ok(diesel::delete(projects::table.find(id)).execute(connection)?)
}
