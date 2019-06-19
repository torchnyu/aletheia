use super::RequestContext;
use crate::types::Token;
use crate::types::{
    Event, EventInsert, EventRequest, LoginRequest, Project, ProjectRequest, Tokenized, User,
    UserRequest,
};

use crate::db::sql_types::*;
use juniper::FieldResult;

pub struct MutationRoot {}

graphql_object!(MutationRoot: RequestContext as "Mutation" |&self| {
    description: "The root mutation object of the schema"

    field login(
        &executor,
        email: String,
        password: String,
    ) -> FieldResult<Tokenized<User>>  {
        let database_context = executor.context().db_context_for_anon_user(ActionType::Read, ActionModifier::One);
        let credentials = LoginRequest {
            email, password
        };
        let user = crate::resolvers::user::login(&credentials, &database_context)?;
        let token = Token::new(&user.email).to_string()?;
        Ok(Tokenized { payload: user, token })
    }

    field create_project(
        &executor,
        project: ProjectRequest,
        token: String
    ) -> FieldResult<Tokenized<Project>> {
        let token = token.parse::<Token>()?;
        let token_string = token.to_string()?;
        let database_context = executor.context().database_context(Resource::Project, Some(&token), ActionType::Create, ActionModifier::Own)?;
        let project = crate::resolvers::project::create(
            &(database_context.get_user()?).email,
            project,
            &database_context
        )?;
        Ok(Tokenized { payload: project, token: token_string })
    }

    field create_event(
        &executor,
        event: EventRequest,
        token: String
    ) -> FieldResult<Tokenized<Event>> {
        let token = token.parse::<Token>()?;
        let token_string = token.to_string()?;
        let database_context = executor.context().database_context(Resource::Event, Some(&token), ActionType::Create, ActionModifier::Own)?;
        let event = crate::resolvers::event::create(
            &(database_context.get_user()?).email,
            EventInsert::from_request(event),
            &database_context
        )?;
        Ok(Tokenized { payload: event, token: token_string })
    }

    field register(
        &executor,
        email: String,
        password: String
    ) -> FieldResult<Tokenized<User>> {
        let user_request = UserRequest {
            display_name: None,
            email,
            password,
        };
        let database_context = executor.context().db_context_for_anon_user(ActionType::Create, ActionModifier::One);
        let user = crate::resolvers::user::create(user_request, &database_context)?;
        let token = Token::new(&user.email).to_string()?;
        Ok(Tokenized { payload: user, token })
    }

});
