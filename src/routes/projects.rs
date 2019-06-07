use crate::db::RequestContext;
use crate::resolvers;
use crate::routes::media::*;
use crate::types::{MediumResponse, Project, ProjectRequest, Token, Tokenized};
use crate::utils::Result;
use rocket::http::{ContentType, Status};
use rocket::response::status::Custom;
use rocket::Data;
use rocket::{get, post};
use rocket_contrib::json::Json;
use std::convert::TryInto;

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

#[post("/images", data = "<data>")]
pub fn upload_image(
    conn: RequestContext,
    content_type: &ContentType,
    token: Token,
    data: Data,
) -> core::result::Result<Json<MediumResponse>, Custom<String>> {
    let boundary = validate_medium_upload(&conn, content_type, &token)?;
    let entries = process_file_upload(boundary, data)?;
    let project_id = get_foreign_key("project_id", &entries)?;
    let user = crate::resolvers::user::get_by_email(&token.uid, &conn).unwrap();
    let medium = process_entries(entries, conn, project_id, Some(user.id))?;
    match medium.try_into() {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err(Custom(Status::InternalServerError, err.to_string())),
    }
}
