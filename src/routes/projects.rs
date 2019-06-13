use crate::db::sql_types::{ActionModifier, ActionType};
use crate::db::RequestContext;
use crate::resolvers;
use crate::types::{Project, ProjectRequest, Token, Tokenized};
use crate::utils::Result;
use rocket::{get, post};
use rocket_contrib::json::Json;

#[get("/")]
pub fn index(context: RequestContext) -> Result<Json<Vec<Project>>> {
    let database_context = context.db_context_for_anon_user(ActionType::Read, ActionModifier::All);
    Ok(Json(resolvers::project::all(&database_context)?))
}

#[post("/", format = "application/json", data = "<project>")]
pub fn create(
    context: RequestContext,
    project: Json<ProjectRequest>,
    token: Token,
) -> Result<Json<Tokenized<Project>>> {
    let project = project.into_inner();
    let database_context = context.db_context_for_anon_user(ActionType::Create, ActionModifier::Own);
    let new_project = resolvers::project::create(&token.uid, project, &database_context)?;
    Ok(Json(Tokenized {
        payload: new_project,
        token: token.to_string()?,
    }))
}
