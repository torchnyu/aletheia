use super::RequestContext;
use crate::types::{Event, Project};
use chrono::naive::NaiveDateTime;
use juniper::FieldResult;

graphql_object!(Event: RequestContext |&self| {
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
        let conn = &executor.context().conn;
        Ok(self.projects(conn)?)
    }
});
