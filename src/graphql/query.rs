use super::RequestContext;
use crate::db::sql_types::{ActionModifier, ActionType, Resource};
use crate::types::{Event, Medium, Project, Token, User};
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

    field userByToken(
        &executor,
        token: String,
    ) -> FieldResult<User> {
        let token = token.parse::<Token>()?;
        let database = &executor.context().database_context(Resource::User, Some(&token), ActionType::Read, ActionModifier::Own)?;
        let user = crate::resolvers::user::get_by_email(&token.uid, &database.conn)?;
        Ok(user)
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
