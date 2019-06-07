use crate::db::models::{LoginRequest, User, UserRequest};
use crate::db::sql_types::{ActionModifier, ActionType};
use crate::db::RequestContext;
use crate::resolvers;
use crate::routes::media::*;
use crate::types::{MediumResponse, Token};
use crate::utils::Result;
use rocket::http::Header;
use rocket::http::{ContentType, Status};
use rocket::response::status::Custom;
use rocket::Data;
use rocket::{get, post, Responder};
use rocket_contrib::json::Json;
use std::convert::TryInto;

#[derive(Responder)]
pub struct AuthenticatedResponse {
    data: Json<User>,
    header: Header<'static>,
}

#[get("/")]
pub fn index(context: RequestContext) -> Result<Json<Vec<User>>> {
    let database_context = context.db_context_for_anon_user(ActionType::Read, ActionModifier::All);
    Ok(Json(resolvers::user::all(&database_context)?))
}

#[post("/", format = "application/json", data = "<user>")]
pub fn create(context: RequestContext, user: Json<UserRequest>) -> Result<Json<User>> {
    let user = user.into_inner();
    let database_context =
        context.db_context_for_anon_user(ActionType::Create, ActionModifier::One);
    Ok(Json(resolvers::user::create(user, &database_context)?))
}

#[post("/login", format = "application/json", data = "<creds>")]
pub fn login(context: RequestContext, creds: Json<LoginRequest>) -> Result<AuthenticatedResponse> {
    let creds = creds.into_inner();
    let database_context = context.db_context_for_anon_user(ActionType::Read, ActionModifier::One);
    let user = resolvers::user::login(&creds, &database_context)?;
    let token = Token::new(&creds.email).to_string()?;
    let response = AuthenticatedResponse {
        data: Json(user),
        header: Header::new("token", token),
    };
    Ok(response)
}

#[post("/profile-picture", data = "<data>")]
pub fn upload_profile_picture(
    conn: RequestContext,
    content_type: &ContentType,
    token: Token,
    data: Data,
) -> core::result::Result<Json<MediumResponse>, Custom<String>> {
    let boundary = validate_medium_upload(&conn, content_type, &token)?;
    let entries = process_file_upload(boundary, data)?;
    // Unwrap cause we know user exists due to validate_medium_upload
    let user = crate::resolvers::user::get_by_email(&token.uid, &conn).unwrap();
    let medium = process_entries(entries, conn, None, Some(user.id))?;
    match medium.try_into() {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err(Custom(Status::InternalServerError, err.to_string())),
    }
}

#[get("/profile-picture")]
pub fn get_profile_picture(
    conn: RequestContext,
    token: Token,
) -> Result<Json<Option<MediumResponse>>> {
    let user = crate::resolvers::user::get_by_email(&token.uid, &conn)?;
    let profile_picture = match user.profile_picture(&conn) {
        Some(pfp) => Some(pfp.try_into()?),
        None => None,
    };
    Ok(Json(profile_picture))
}
