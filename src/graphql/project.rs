use super::RequestContext;
use crate::db::sql_types::{ActionModifier, ActionType};
use crate::types::{Medium, Project, User};
use heck::TitleCase;

graphql_object!(Project: RequestContext |&self| {
    description: "A hackathon project"

    field id(&executor) -> i32 {
        self.id
    }

    field name(&executor) -> &str {
        &self.name
    }

    field repository_url(&executor) -> &str {
        &self.repository_url
    }

    field slug(&executor) -> &str {
        &self.slug
    }

    field title(&executor) -> String {
        self.name.to_title_case()
    }

    field description(&executor) -> Option<&str> {
        match &self.description {
            Some(desc) => Some(desc.as_str()),
            None => None
        }
    }

    field media(&executor) -> Vec<Medium> {
        let database_context = executor.context().db_context_anon(ActionType::Read, ActionModifier::All);
        self.media(&database_context)
    }

    field contributors(&executor) -> Vec<User> {
        let database_context = executor.context().db_context_anon(ActionType::Read, ActionModifier::All);
        self.contributors(&database_context)
    }
});
