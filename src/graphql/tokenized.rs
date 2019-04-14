use super::RequestContext;
use crate::types::{Event, Project, Tokenized, User};

graphql_object!(Tokenized<Project>: RequestContext as "AuthenticatedProject" |&self| {
    field token() -> &str {
        &self.token
    }

    field project() -> &Project {
        &self.payload
    }
});

graphql_object!(Tokenized<Event>: RequestContext as "AuthenticatedEvent" |&self| {
    field token() -> &str {
        &self.token
    }

    field event() -> &Event {
        &self.payload
    }
});

graphql_object!(Tokenized<User>: RequestContext as "AuthenticatedUser" |&self| {
    field token() -> &str {
        &self.token
    }

    field user() -> &User {
        &self.payload
    }
});
