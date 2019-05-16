use crate::db::models::{LoginRequest, User, UserRequest};
use crate::db::sql_types::{ActionModifier, ActionType};
use crate::db::RequestContext;
use crate::resolvers;
use crate::types::Token;
use crate::utils::Result;
use rocket::http::Header;
use rocket::{get, post, Responder};
use rocket_contrib::json::Json;

#[derive(Responder)]
pub struct AuthenticatedResponse {
    data: Json<User>,
    header: Header<'static>,
}

#[get("/")]
pub fn index(context: RequestContext) -> Result<Json<Vec<User>>> {
    let database_context = context.database_context(None, ActionType::Read, ActionModifier::All);
    Ok(Json(resolvers::user::all(&database_context)?))
}

#[post("/", format = "application/json", data = "<user>")]
pub fn create(context: RequestContext, user: Json<UserRequest>) -> Result<Json<User>> {
    let user = user.into_inner();
    let database_context = context.database_context(None, ActionType::Create, ActionModifier::One);
    Ok(Json(resolvers::user::create(user, &database_context)?))
}

#[post("/login", format = "application/json", data = "<creds>")]
pub fn login(context: RequestContext, creds: Json<LoginRequest>) -> Result<AuthenticatedResponse> {
    let creds = creds.into_inner();
    let database_context = context.database_context(None, ActionType::Read, ActionModifier::One);
    let user = resolvers::user::login(&creds, &database_context)?;
    let token = Token::new(&creds.email).to_string()?;
    let response = AuthenticatedResponse {
        data: Json(user),
        header: Header::new("token", token),
    };
    Ok(response)
}
