use crate::db::sql_types::{ActionModifier, ActionType};
use crate::db::RequestContext;
use crate::resolvers;
use crate::types::{Submission, SubmissionInsert, Token, Tokenized};
use crate::utils::Result;
use rocket::{get, post};
use rocket_contrib::json::Json;

#[get("/")]
pub fn index(context: RequestContext) -> Result<Json<Vec<Submission>>> {
    let database_context = context.db_context_for_anon_user(ActionType::Read, ActionModifier::All);
    Ok(Json(resolvers::submission::all(&database_context)?))
}

#[post("/", format = "application/json", data = "<submission>")]
pub fn create(
    context: RequestContext,
    submission: Json<SubmissionInsert>,
    token: Token,
) -> Result<Json<Tokenized<Submission>>> {
    let submission = submission.into_inner();
    let database_context = context.db_context_for_anon_user(ActionType::Create, ActionModifier::Own);
    Ok(Json(Tokenized {
        payload: resolvers::submission::insert(submission, &database_context)?,
        token: token.to_string()?,
    }))
}
