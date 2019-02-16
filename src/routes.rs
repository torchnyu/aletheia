pub mod projects {
    use crate::models::Project;
    use crate::types::{DbConn, InsertableProject, Result};
    use rocket::{get, post};
    use rocket_contrib::json::Json;

    #[get("/")]
    pub fn index(conn: DbConn) -> Result<Json<Vec<Project>>> {
        Ok(Json(crate::controllers::projects_controller::all(&conn)?))
    }

    #[post("/", format = "application/json", data = "<project>")]
    pub fn create(conn: DbConn, project: Json<InsertableProject>) -> Result<Json<Project>> {
        let project = project.into_inner();
        Ok(Json(crate::controllers::projects_controller::insert(
            project, &conn,
        )?))
    }
}

pub mod users {
    use crate::models::{LoginRequest, User, UserRequest, UserResponse};
    use crate::types::{DbConn, Result};
    use rocket::http::Header;
    use rocket::{get, post, Response};
    use rocket_contrib::json::Json;

    #[get("/")]
    pub fn index(conn: DbConn) -> Result<Json<Vec<UserResponse>>> {
        Ok(Json(crate::controllers::users_controller::all(&conn)?))
    }

    #[post("/", format = "application/json", data = "<user>")]
    pub fn create(conn: DbConn, user: Json<UserRequest>) -> Result<Json<UserResponse>> {
        let user = user.into_inner();
        Ok(Json(crate::controllers::users_controller::create(
            user, &conn,
        )?))
    }

    #[post("/login", format = "application/json", data = "<creds>")]
    pub fn login(conn: DbConn, creds: Json<LoginRequest>) -> Result<Json<UserResponse>> {
        let creds = creds.into_inner();
        let user = crate::controllers::users_controller::login(&creds, &conn)?;
        let token = crate::tokens::create_token(&creds.email)?;
        Ok(Json(user))
    }
}
