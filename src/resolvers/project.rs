use crate::db::models::{
    Project, ProjectInsert, ProjectRequest, Submission, SubmissionInsert, User,
};
use crate::db::schema::users::columns;
use crate::db::schema::{events, projects, submissions, users};
use crate::utils::*;
use diesel::dsl::any;
use diesel::prelude::*;
use diesel::BelongingToDsl;
use rocket_contrib::databases::diesel;
use slug::slugify;

pub fn all(conn: &diesel::PgConnection) -> Result<Vec<Project>> {
    Ok(projects::table.load::<Project>(&*conn)?)
}

pub fn get(id: i32, conn: &diesel::PgConnection) -> Result<Project> {
    Ok(projects::table.find(id).get_result::<Project>(conn)?)
}

pub fn get_by_slug(slug: &str, conn: &diesel::PgConnection) -> Result<Project> {
    Ok(projects::table
        .filter(projects::slug.eq(slug))
        .first(conn)?)
}

pub fn create(
    email: &str,
    project: ProjectRequest,
    conn: &diesel::PgConnection,
) -> Result<Project> {
    conn.transaction::<_, _, _>(|| {
        let event_id = events::table
            .filter(events::slug.eq(project.event_slug))
            .select(events::id)
            .first(conn)?;
        let slug = slugify(&project.name);
        let project = ProjectInsert {
            name: project.name,
            repository_url: project.repository_url,
            description: project.description,
            event_id,
            slug,
        };
        // Create project
        let project: Project = diesel::insert_into(projects::table)
            .values(&project)
            .get_result(conn)?;
        // Get id from users table. We could probably also call user
        // resolver. Idk if that's better (could cause circular
        // dependencies)
        let user_id = users::table
            .filter(users::email.eq(email))
            .select(users::id)
            .first(conn)?;

        let submission = SubmissionInsert {
            user_id,
            project_id: project.id,
        };
        // We need the variable for typechecking to infer the
        // Submission type. I could use the turbofish (::<>) but this
        // is a little cleaner imo
        let _submission: Submission = diesel::insert_into(submissions::table)
            .values(&submission)
            .get_result(conn)?;
        Ok(project)
    })
}

pub fn update(id: i32, person: Project, conn: &diesel::PgConnection) -> Result<Project> {
    Ok(diesel::update(projects::table.find(id))
        .set(&person)
        .get_result(conn)?)
}

pub fn delete(id: i32, conn: &diesel::PgConnection) -> Result<usize> {
    Ok(diesel::delete(projects::table.find(id)).execute(conn)?)
}

impl Project {
    pub fn contributors(&self, conn: &diesel::PgConnection) -> Vec<User> {
        let user_ids = Submission::belonging_to(self).select(submissions::user_id);
        users::table
            .filter(users::id.eq(any(user_ids)))
            .select((columns::id, columns::display_name, columns::email))
            .load::<User>(conn)
            .expect("Could not load contributors")
    }
}
