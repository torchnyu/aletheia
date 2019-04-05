use crate::resolvers;
use crate::types::{LoginRequest, Token, User, UserRequest, Connection};
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
pub fn index(conn: Connection) -> Result<Json<Vec<User>>> {
    Ok(Json(resolvers::user::all(&conn)?))
}

#[post("/", format = "application/json", data = "<user>")]
pub fn create(conn: Connection, user: Json<UserRequest>) -> Result<Json<User>> {
    let user = user.into_inner();
    Ok(Json(resolvers::user::create(user, &conn)?))
}

#[post("/login", format = "application/json", data = "<creds>")]
pub fn login(conn: Connection, creds: Json<LoginRequest>) -> Result<AuthenticatedResponse> {
    let creds = creds.into_inner();
    let user = resolvers::user::login(&creds, &conn)?;
    let token = Token::new(&creds.email).to_string()?;
    let response = AuthenticatedResponse {
        data: Json(user),
        header: Header::new("token", token),
    };
    Ok(response)
}
