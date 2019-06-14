use crate::db::connection::DatabaseContext;
use crate::db::models::{
    Medium, Project, ProjectInsert, ProjectRequest, Submission, SubmissionInsert, User,
};
use crate::db::schema::users::columns;
use crate::db::schema::{events, media, projects, submissions, users};
use crate::utils::*;
use diesel::dsl::any;
use diesel::prelude::*;
use diesel::BelongingToDsl;
use rocket_contrib::databases::diesel;
use slug::slugify;

pub fn all(db: &DatabaseContext) -> Result<Vec<Project>> {
    Ok(projects::table.load::<Project>(db.conn)?)
}

pub fn get(id: i32, db: &DatabaseContext) -> Result<Project> {
    Ok(projects::table.find(id).get_result::<Project>(db.conn)?)
}

pub fn get_by_slug_and_event(
    slug: &str,
    event_slug: &str,
    db: &DatabaseContext,
) -> Result<Project> {
    let event_id: i32 = events::table
        .filter(events::slug.eq(event_slug))
        .select(events::id)
        .first(db.conn)?;
    Ok(projects::table
        .filter(projects::event_id.eq(event_id))
        .filter(projects::slug.eq(slug))
        .first(db.conn)?)
}

pub fn create(email: &str, project: ProjectRequest, db: &DatabaseContext) -> Result<Project> {
    db.conn.transaction::<_, _, _>(|| {
        let event_id = events::table
            .filter(events::slug.eq(project.event_slug))
            .select(events::id)
            .first(db.conn)?;
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
            .get_result(db.conn)?;
        // Get id from users table. We could probably also call user
        // resolver. Idk if that's better (could cause circular
        // dependencies)
        let user_id = users::table
            .filter(users::email.eq(email))
            .select(users::id)
            .first(db.conn)?;

        let submission = SubmissionInsert {
            user_id,
            project_id: project.id,
        };
        // We need the variable for typechecking to infer the
        // Submission type. I could use the turbofish (::<>) but this
        // is a little cleaner imo
        let _submission: Submission = diesel::insert_into(submissions::table)
            .values(&submission)
            .get_result(db.conn)?;
        Ok(project)
    })
}

pub fn update(id: i32, person: Project, db: &DatabaseContext) -> Result<Project> {
    Ok(diesel::update(projects::table.find(id))
        .set(&person)
        .get_result(db.conn)?)
}

pub fn delete(id: i32, db: &DatabaseContext) -> Result<usize> {
    Ok(diesel::delete(projects::table.find(id)).execute(db.conn)?)
}

impl Project {
    pub fn contributors(&self, db: &DatabaseContext) -> Vec<User> {
        let user_ids = Submission::belonging_to(self).select(submissions::user_id);
        users::table
            .filter(users::id.eq(any(user_ids)))
            .select((columns::id, columns::display_name, columns::email))
            .load::<User>(db.conn)
            .expect("Could not load contributors")
    }

    pub fn media(&self, db: &DatabaseContext) -> Vec<Medium> {
        media::table
            .filter(media::project_id.eq(self.id))
            .load::<Medium>(db.conn)
            .expect("Could not load media")
    }
}
