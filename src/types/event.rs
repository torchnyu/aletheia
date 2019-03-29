use super::Context;
use crate::resolvers;
use crate::schema::*;
use crate::types::Project;
use chrono::naive::NaiveDateTime;
use diesel::{self, AsChangeset, Queryable};
use juniper::FieldResult;
use serde_derive::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "events"]
pub struct Event {
    id: i32,
    name: String,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    description: Option<String>,
    slug: String,
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
        Ok(resolvers::event::projects(self, database)?)
    }
});
