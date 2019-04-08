use super::Context;
use crate::db::schema::*;
use crate::db::orm::Project;
use chrono::naive::NaiveDateTime;
use diesel::{self, AsChangeset, Queryable};
use juniper::FieldResult;
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

graphql_object!(Event: Context |&self| {
    description: "A event, e.g. hackathon, code challenge, etc."

    field id(&executor) -> i32 {
        self.id
    }

    field name(&executor) -> &str {
        &self.name
    }

    field start_time(&executor) -> &NaiveDateTime {
        &self.start_time
    }

    field end_time(&executor) -> &NaiveDateTime {
        &self.end_time
    }

    field description(&executor) -> Option<&str> {
        match &self.description {
            Some(desc) => Some(desc.as_str()),
            None => None
        }
    }

    field slug(&executor) -> &str {
        &self.slug
    }

    field projects(&executor) -> FieldResult<Vec<Project>> {
        let database: &diesel::PgConnection = &executor.context().database;
        Ok(self.projects(database)?)
    }
});
