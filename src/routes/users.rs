use crate::authorization::validate;
use crate::db::models::{LoginRequest, User, UserRequest};
use crate::db::sql_types::{ActionModifier, ActionType, Resource};
use crate::db::RequestContext;
use crate::resolvers;
use crate::routes::media::*;
use crate::types::{MediumResponse, ResetPasswordParams, SendResetPasswordParams, Token};
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
pub fn index(context: RequestContext) -> crate::utils::Result<Json<Vec<User>>> {
    Ok(Json(resolvers::user::all(&context.conn)?))
}

#[post("/", format = "application/json", data = "<user>")]
pub fn create(
    context: RequestContext,
    user: Json<UserRequest>,
) -> crate::utils::Result<Json<User>> {
    let user = user.into_inner();
    Ok(Json(resolvers::user::create(user, &context.conn)?))
}

#[post("/login", format = "application/json", data = "<creds>")]
pub fn login(
    context: RequestContext,
    creds: Json<LoginRequest>,
) -> crate::utils::Result<AuthenticatedResponse> {
    let mut creds = creds.into_inner();
    let lowercase_email = creds.email.to_ascii_lowercase();
    creds.email = lowercase_email;
    let user = resolvers::user::login(&creds, &context.conn)?;
    let token = Token::new(&creds.email).to_string()?;
    let response = AuthenticatedResponse {
        data: Json(user),
        header: Header::new("token", token),
    };
    Ok(response)
}

#[post("/profile-picture", data = "<data>")]
pub fn upload_profile_picture(
    ctx: RequestContext,
    content_type: &ContentType,
    token: Token,
    data: Data,
) -> Result<Json<MediumResponse>, Custom<String>> {
    let boundary = validate_medium_upload(&ctx, content_type, &token)?;
    let entries = process_file_upload(boundary, data)?;
    // Unwrap cause we know user exists due to validate_medium_upload
    let user = crate::resolvers::user::get_by_email(&token.uid, &ctx.conn).unwrap();
    let medium = process_entries(entries, &ctx, None, Some(user.id))?;
    match medium.try_into() {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err(Custom(Status::InternalServerError, err.to_string())),
    }
}

#[get("/profile-picture")]
pub fn get_profile_picture(
    ctx: RequestContext,
    token: Token,
) -> Result<Option<Json<MediumResponse>>, Custom<String>> {
    match validate(
        &ctx.conn,
        &token,
        Resource::Medium,
        ActionType::Read,
        ActionModifier::Own,
    ) {
        Ok(ctx) => ctx,
        Err(err) => return Err(Custom(Status::Unauthorized, err.to_string())),
    };
    let user = crate::resolvers::user::get_by_email(&token.uid, &ctx.conn).unwrap();
    let profile_picture = match user.profile_picture(&ctx.conn) {
        Some(pfp) => pfp,
        None => return Ok(None),
    };
    match profile_picture.try_into() {
        Ok(pfp) => Ok(Some(Json(pfp))),
        Err(err) => Err(Custom(Status::InternalServerError, err.to_string())),
    }
}

#[post(
    "/send-reset-password-email",
    format = "application/json",
    data = "<params>"
)]
pub fn send_reset_password_email(
    ctx: RequestContext,
    params: Json<SendResetPasswordParams>,
) -> Result<(), Custom<String>> {
    let params = params.into_inner();
    let email = &params.email.to_ascii_lowercase();
    match resolvers::user::send_reset_email(&email, &params.domain, &ctx.conn) {
        Ok(()) => Ok(()),
        Err(err) => Err(Custom(Status::InternalServerError, err.to_string())),
    }
}

#[post("/reset-password", format = "application/json", data = "<params>")]
pub fn reset_password(
    ctx: RequestContext,
    params: Json<ResetPasswordParams>,
) -> Result<AuthenticatedResponse, Custom<String>> {
    let params = params.into_inner();
    let email = &params.email.to_ascii_lowercase();
    match resolvers::user::reset_password(email, &params.password, &params.key, &ctx.conn) {
        Ok(user) => {
            let token = match Token::new(&user.email).to_string() {
                Err(err) => return Err(Custom(Status::InternalServerError, err.to_string())),
                Ok(token) => token,
            };
            Ok(AuthenticatedResponse {
                data: Json(user),
                header: Header::new("token", token),
            })
        }
        Err(err) => Err(Custom(Status::InternalServerError, err.to_string())),
    }
}
