use super::RequestContext;
use crate::db::sql_types::{ActionModifier, ActionType};
use crate::types::{Event, Medium, Project, User};
use juniper::FieldResult;

pub struct QueryRoot {}

graphql_object!(QueryRoot: RequestContext as "Query" |&self| {
    description: "The root query object of the schema"

    field users(
        &executor
    ) -> FieldResult<Vec<User>> {
        let database_context = executor.context().db_context_for_anon_user(ActionType::Read, ActionModifier::All);
        Ok(crate::resolvers::user::all(&database_context)?)
    }

    field projects(
        &executor
    ) -> FieldResult<Vec<Project>> {
        let database_context = executor.context().db_context_for_anon_user(ActionType::Read, ActionModifier::All);
        Ok(crate::resolvers::project::all(&database_context)?)
    }

    field media(
        &executor
    ) -> FieldResult<Vec<Medium>> {
        let database = &executor.context().db_context_for_anon_user(ActionType::Read, ActionModifier::All);
        Ok(crate::resolvers::medium::all(&database)?)
    }

    field projectBySlugAndEvent(
        &executor,
        slug: String,
        event_slug: String,
    ) -> FieldResult<Project> {
        let database_context = executor.context().db_context_for_anon_user(ActionType::Read, ActionModifier::All);
        Ok(crate::resolvers::project::get_by_slug_and_event(&slug, &event_slug, &database_context)?)
    }

    field eventBySlug(
        &executor,
        slug: String
    ) -> FieldResult<Event> {
        let database_context = executor.context().db_context_for_anon_user(ActionType::Read, ActionModifier::All);
        Ok(crate::resolvers::event::get_by_slug(&slug, &database_context)?)
    }

    field events(&executor) -> FieldResult<Vec<Event>> {
        let database_context = executor.context().db_context_for_anon_user(ActionType::Read, ActionModifier::All);
        Ok(crate::resolvers::event::all(&database_context)?)
    }

});
