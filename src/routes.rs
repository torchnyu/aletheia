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

pub mod contributors {
    use crate::models::Contributor;
    use crate::types::{DbConn, InsertableContributor, Result};
    use rocket::{get, post};
    use rocket_contrib::json::Json;

    #[get("/")]
    pub fn index(conn: DbConn) -> Result<Json<Vec<Contributor>>> {
        Ok(Json(crate::controllers::contributors_controller::all(
            &conn,
        )?))
    }

    #[post("/", format = "application/json", data = "<contributor>")]
    pub fn create(
        conn: DbConn,
        contributor: Json<InsertableContributor>,
    ) -> Result<Json<Contributor>> {
        let contributor = contributor.into_inner();
        Ok(Json(crate::controllers::contributors_controller::insert(
            contributor,
            &conn,
        )?))
    }
}
