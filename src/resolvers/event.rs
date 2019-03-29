use crate::schema::events;
use crate::types::{Event, Project};
use crate::utils::*;
use diesel::prelude::*;
use diesel::BelongingToDsl;

pub fn all(conn: &diesel::PgConnection) -> Result<Vec<Event>> {
    Ok(events::table.load::<Event>(&*conn)?)
}

pub fn projects(event: &Event, conn: &diesel::PgConnection) -> Result<Vec<Project>> {
    Ok(Project::belonging_to(event).load::<Project>(&*conn)?)
}

pub fn get_by_slug(slug: &str, conn: &diesel::PgConnection) -> Result<Event> {
    Ok(events::table.filter(events::slug.eq(slug)).first(conn)?)
}
