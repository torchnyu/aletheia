use crate::db::RequestContext;
use crate::resolvers;
use crate::types::{Project, ProjectRequest, Token, Tokenized};
use crate::utils::Result;
use rocket::{get, post};
use rocket_contrib::json::Json;

#[get("/")]
pub fn index(context: RequestContext) -> Result<Json<Vec<Project>>> {
    Ok(Json(resolvers::project::all(&context.conn)?))
}

#[post("/", format = "application/json", data = "<project>")]
pub fn create(
    context: RequestContext,
    project: Json<ProjectRequest>,
    token: Token,
) -> Result<Json<Tokenized<Project>>> {
    let project = project.into_inner();
    let new_project = resolvers::project::create(&token.uid, project, &context.conn)?;
    Ok(Json(Tokenized {
        payload: new_project,
        token: token.to_string()?,
    }))
}
