use crate::db::Connection;
use crate::resolvers;
use crate::tokens::Token;
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
    let token = project_with_token.token.parse::<Token>()?;
    let new_token = token.validate()?;
    let new_project = resolvers::project::create(
        &new_token,
        ProjectInsert::from_request(project_with_token.payload),
        &conn,
    )?;
    Ok(Json(Tokenized {
        payload: new_project,
        token: new_token.to_string()?,
    }))
}
