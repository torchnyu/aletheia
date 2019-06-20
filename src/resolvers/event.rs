use crate::db::models::{Event, EventInsert, Project, UserEvent, UserEventInsert};
use crate::db::schema::{events, user_events, users};
use crate::db::PgConnection;
use crate::utils::*;
use diesel::prelude::*;
use diesel::BelongingToDsl;

pub fn all(db: &PgConnection) -> Result<Vec<Event>> {
    Ok(events::table.load::<Event>(db)?)
}

impl Event {
    pub fn projects(&self, db: &PgConnection) -> Result<Vec<Project>> {
        Ok(Project::belonging_to(self).load::<Project>(db)?)
    }
}

pub fn get_by_slug(slug: &str, db: &PgConnection) -> Result<Event> {
    Ok(events::table.filter(events::slug.eq(slug)).first(db)?)
}

pub fn create(email: &str, event: EventInsert, db: &PgConnection) -> Result<Event> {
    db.transaction::<_, _, _>(|| {
        // Create project
        let event: Event = diesel::insert_into(events::table)
            .values(&event)
            .get_result(db)?;
        // Get id from users table. We could probably also call user
        // resolver. Idk if that's better (could cause circular
        // dependencies)
        let user_id = users::table
            .filter(users::email.eq(email))
            .select(users::id)
            .first(db)?;

        let user_event = UserEventInsert {
            user_id,
            event_id: event.id,
        };
        // We need the variable for typechecking to infer the
        // Submission type. I could use the turbofish (::<>) but this
        // is a little cleaner imo
        let _user_event: UserEvent = diesel::insert_into(user_events::table)
            .values(&user_event)
            .get_result(db)?;
        Ok(event)
    })
}
