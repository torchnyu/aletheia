use super::RequestContext;
use crate::types::{Event, Medium, Project, Token, User};
use juniper::FieldResult;

pub struct QueryRoot {}

graphql_object!(QueryRoot: RequestContext as "Query" |&self| {
    description: "The root query object of the schema"

    field users(
        &executor
    ) -> FieldResult<Vec<User>> {
        let conn = &executor.context().conn;
        Ok(crate::resolvers::user::all(conn)?)
    }

    field projects(
        &executor
    ) -> FieldResult<Vec<Project>> {
        let conn = &executor.context().conn;
        Ok(crate::resolvers::project::all(conn)?)
    }

    field media(
        &executor
    ) -> FieldResult<Vec<Medium>> {
        let conn = &executor.context().conn;
        Ok(crate::resolvers::medium::all(conn)?)
    }

    field userByToken(
        &executor,
        token: String,
    ) -> FieldResult<User> {
        let database = &executor.context().conn;
        let token = token.parse::<Token>()?;
        crate::authorization::validate(
            &database,
            &token,
            Resource::User,
            ActionType::Read,
            ActionModifier::Own
        )?;
        // We're fetching the user in the validation, so this is
        // technically a second fetch, but I don't want to break the
        // conceptual integrity of validate
        let user = crate::resolvers::user::get_by_email(&token.uid, database)?;
        Ok(user)
    }

    field projectBySlugAndEvent(
        &executor,
        slug: String,
        event_slug: String,
    ) -> FieldResult<Project> {
        let conn = &executor.context().conn;
        Ok(crate::resolvers::project::get_by_slug_and_event(&slug, &event_slug, conn)?)
    }

    field eventBySlug(
        &executor,
        slug: String
    ) -> FieldResult<Event> {
        let conn = &executor.context().conn;
        Ok(crate::resolvers::event::get_by_slug(&slug, conn)?)
    }

    field events(&executor) -> FieldResult<Vec<Event>> {
        let conn = &executor.context().conn;
        Ok(crate::resolvers::event::all(conn)?)
    }

});
