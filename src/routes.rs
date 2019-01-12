use crate::controllers;
use crate::models::Project;
use crate::types::{DbConn, InsertableProject, Result};
use rocket_contrib::json::Json;

#[get("/")]
pub fn index(conn: DbConn) -> Result<Json<Vec<Project>>> {
    Ok(Json(controllers::all(&conn)?))
}

#[post("/", format = "application/json", data = "<project>")]
pub fn create(conn: DbConn, project: Json<InsertableProject>) -> Result<Json<Project>> {
    let project = project.into_inner();
    Ok(Json(controllers::insert(project, &conn)?))
}
