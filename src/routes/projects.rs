use crate::db::Connection;
use crate::resolvers;
use crate::types::{Project, ProjectInsert, ProjectRequest};
use crate::utils::Result;
use rocket::{get, post};
use rocket_contrib::json::Json;

#[get("/")]
pub fn index(conn: Connection) -> Result<Json<Vec<Project>>> {
    Ok(Json(resolvers::project::all(&conn)?))
}

#[post("/", format = "application/json", data = "<project>")]
pub fn create(conn: Connection, project: Json<ProjectRequest>) -> Result<Json<Project>> {
    let project = project.into_inner();
    Ok(Json(resolvers::project::insert(
        ProjectInsert::from_request(project),
        &conn,
    )?))
}
