use crate::db::Connection;
use crate::resolvers;
use crate::types::{Project, ProjectInsert, ProjectRequest, Token, Tokenized};
use crate::utils::Result;
use rocket::{get, post};
use rocket_contrib::json::Json;

#[get("/")]
pub fn index(conn: Connection) -> Result<Json<Vec<Project>>> {
    Ok(Json(resolvers::project::all(&conn)?))
}

#[post("/", format = "application/json", data = "<project>")]
pub fn create(
    conn: Connection,
    project: Json<ProjectRequest>,
    token: Token,
) -> Result<Json<Tokenized<Project>>> {
    let project = project.into_inner();
    let new_project =
        resolvers::project::create(&token.uid, ProjectInsert::from_request(project), &conn)?;
    Ok(Json(Tokenized {
        payload: new_project,
        token: token.to_string()?,
    }))
}
