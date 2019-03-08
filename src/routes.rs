pub mod projects {
    use crate::db::Connection;
    use crate::types::{Project, ProjectInsert};
    use crate::utils::Result;
    use rocket::{get, post};
    use rocket_contrib::json::Json;

    #[get("/")]
    pub fn index(conn: Connection) -> Result<Json<Vec<Project>>> {
        Ok(Json(crate::controllers::projects_controller::all(&conn)?))
    }

    #[post("/", format = "application/json", data = "<project>")]
    pub fn create(conn: Connection, project: Json<ProjectInsert>) -> Result<Json<Project>> {
        let project = project.into_inner();
        Ok(Json(crate::controllers::projects_controller::insert(
            project, &conn,
        )?))
    }
}

pub mod submissions {
    use crate::db::Connection;
    use crate::types::{Submission, SubmissionInsert};
    use crate::utils::Result;
    use rocket::{get, post};
    use rocket_contrib::json::Json;

    #[get("/")]
    pub fn index(conn: Connection) -> Result<Json<Vec<Submission>>> {
        Ok(Json(crate::controllers::submissions_controller::all(
            &conn,
        )?))
    }

    #[post("/", format = "application/json", data = "<submission>")]
    pub fn create(
        conn: Connection,
        submission: Json<SubmissionInsert>,
    ) -> Result<Json<Submission>> {
        let submission = submission.into_inner();
        Ok(Json(crate::controllers::submissions_controller::insert(
            submission, &conn,
        )?))
    }
}

pub mod users {
    use crate::db::Connection;
    use crate::types::{LoginRequest, UserRequest, UserResponse};
    use crate::utils::Result;
    use rocket::http::Header;
    use rocket::{get, post, Responder};
    use rocket_contrib::json::Json;

    #[derive(Responder)]
    pub struct AuthenticatedResponse {
        data: Json<UserResponse>,
        header: Header<'static>,
    }

    #[get("/")]
    pub fn index(conn: Connection) -> Result<Json<Vec<UserResponse>>> {
        Ok(Json(crate::controllers::users_controller::all(&conn)?))
    }

    #[post("/", format = "application/json", data = "<user>")]
    pub fn create(conn: Connection, user: Json<UserRequest>) -> Result<Json<UserResponse>> {
        let user = user.into_inner();
        Ok(Json(crate::controllers::users_controller::create(
            user, &conn,
        )?))
    }

    #[post("/login", format = "application/json", data = "<creds>")]
    pub fn login(conn: Connection, creds: Json<LoginRequest>) -> Result<AuthenticatedResponse> {
        let creds = creds.into_inner();
        let user = crate::controllers::users_controller::login(&creds, &conn)?;
        let token = crate::tokens::create_token(&creds.email)?;
        let response = AuthenticatedResponse {
            data: Json(user),
            header: Header::new("token", token),
        };
        Ok(response)
    }
}
