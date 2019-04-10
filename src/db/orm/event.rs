use crate::db::schema::*;
use chrono::naive::NaiveDateTime;
use diesel::{self, AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};
use slug::slugify;

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "events"]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub description: Option<String>,
    pub slug: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "events"]
pub struct EventInsert {
    pub name: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub description: Option<String>,
    pub slug: String,
}

#[derive(Serialize, Deserialize, GraphQLInputObject)]
pub struct EventRequest {
    pub name: String,
    // Damn GraphQL doesn't let me use i64. If this bites me in the
    // ass in 2038 I'm gonna be pissed
    pub start_time: i32,
    pub end_time: i32,
    pub description: Option<String>,
}

impl EventInsert {
    pub fn from_request(request: EventRequest) -> EventInsert {
        let slug = slugify(&request.name);
        EventInsert {
            name: request.name,
            start_time: NaiveDateTime::from_timestamp(request.end_time.into(), 0),
            end_time: NaiveDateTime::from_timestamp(request.start_time.into(), 0),
            description: request.description,
            slug,
        }
    }
}
