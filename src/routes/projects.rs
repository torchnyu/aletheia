use crate::db::Connection;
use crate::resolvers;
use crate::tokens::Claims;
use crate::types::{Project, ProjectInsert, ProjectRequest, Tokenized};
use crate::utils::Result;
use rocket::{get, post};
use rocket_contrib::json::Json;

#[get("/")]
pub fn index(conn: Connection) -> Result<Json<Vec<Project>>> {
    Ok(Json(resolvers::project::all(&conn)?))
}

#[post("/", format = "application/json", data = "<project_with_token>")]
pub fn create(
    conn: Connection,
    project_with_token: Json<Tokenized<ProjectRequest>>,
) -> Result<Json<Tokenized<Project>>> {
    let project_with_token = project_with_token.into_inner();
    let token = project_with_token.token.parse::<Claims>()?;
    let new_token = token.validate()?.to_string()?;
    let new_project = resolvers::project::insert(
        ProjectInsert::from_request(project_with_token.payload),
        &conn,
    )?;
    Ok(Json(Tokenized {
        payload: new_project,
        token: new_token,
    }))
}
