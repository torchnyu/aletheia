use crate::db::Connection;
use crate::resolvers;
use crate::types::{Submission, SubmissionInsert};
use crate::utils::Result;
use rocket::{get, post};
use rocket_contrib::json::Json;

#[get("/")]
pub fn index(conn: Connection) -> Result<Json<Vec<Submission>>> {
    Ok(Json(resolvers::submission::all(&conn)?))
}

#[post("/", format = "application/json", data = "<submission>")]
pub fn create(conn: Connection, submission: Json<SubmissionInsert>) -> Result<Json<Submission>> {
    let submission = submission.into_inner();
    Ok(Json(resolvers::submission::insert(submission, &conn)?))
}
