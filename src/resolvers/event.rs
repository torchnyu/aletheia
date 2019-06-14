use crate::db::connection::DatabaseContext;
use crate::db::models::{Event, EventInsert, Project, UserEvent, UserEventInsert};
use crate::db::schema::{events, user_events, users};
use crate::utils::*;
use diesel::prelude::*;
use diesel::BelongingToDsl;

pub fn all(db: &DatabaseContext) -> Result<Vec<Event>> {
    Ok(events::table.load::<Event>(db.conn)?)
}

impl Event {
    pub fn projects(&self, db: &DatabaseContext) -> Result<Vec<Project>> {
        Ok(Project::belonging_to(self).load::<Project>(db.conn)?)
    }
}

pub fn get_by_slug(slug: &str, db: &DatabaseContext) -> Result<Event> {
    Ok(events::table.filter(events::slug.eq(slug)).first(db.conn)?)
}

pub fn create(email: &str, event: EventInsert, db: &DatabaseContext) -> Result<Event> {
    db.conn.transaction::<_, _, _>(|| {
        // Create project
        let event: Event = diesel::insert_into(events::table)
            .values(&event)
            .get_result(db.conn)?;
        // Get id from users table. We could probably also call user
        // resolver. Idk if that's better (could cause circular
        // dependencies)
        let user_id = users::table
            .filter(users::email.eq(email))
            .select(users::id)
            .first(db.conn)?;

        let user_event = UserEventInsert {
            user_id,
            event_id: event.id,
        };
        // We need the variable for typechecking to infer the
        // Submission type. I could use the turbofish (::<>) but this
        // is a little cleaner imo
        let _user_event: UserEvent = diesel::insert_into(user_events::table)
            .values(&user_event)
            .get_result(db.conn)?;
        Ok(event)
    })
}
