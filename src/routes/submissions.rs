use crate::db::RequestContext;
use crate::resolvers;
use crate::types::{Submission, SubmissionInsert, Token, Tokenized};
use crate::utils::Result;
use rocket::{get, post};
use rocket_contrib::json::Json;

#[get("/")]
pub fn index(conn: RequestContext) -> Result<Json<Vec<Submission>>> {
    Ok(Json(resolvers::submission::all(&conn)?))
}

#[post("/", format = "application/json", data = "<submission>")]
pub fn create(
    conn: RequestContext,
    submission: Json<SubmissionInsert>,
    token: Token,
) -> Result<Json<Tokenized<Submission>>> {
    let submission = submission.into_inner();
    Ok(Json(Tokenized {
        payload: resolvers::submission::insert(submission, &conn)?,
        token: token.to_string()?,
    }))
}
