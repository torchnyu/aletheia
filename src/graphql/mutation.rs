use super::RequestContext;
use crate::types::Token;
use crate::types::{
    Event, EventInsert, EventRequest, LoginRequest, Project, ProjectInsert, ProjectRequest,
    Tokenized, User, UserRequest,
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
        let database = &executor.context().conn;
        let credentials = LoginRequest {
            email, password
        };
        let user = crate::resolvers::user::login(&credentials, &database)?;
        let token = Token::new(&user.email).to_string()?;
        Ok(Tokenized { payload: user, token })
    }

    field create_project(
        &executor,
        project: ProjectRequest,
        token: String
    ) -> FieldResult<Tokenized<Project>> {
        let token = token.parse::<Token>()?;
        let database = &executor.context().conn;
        crate::authorization::validate(
            &database,
            &token,
            "project".to_string(),
            ActionType::Create,
            ActionModifier::Own
        )?;
        let project = crate::resolvers::project::create(
            &token.uid,
            ProjectInsert::from_request(project),
            database
        )?;
        Ok(Tokenized { payload: project, token: token.to_string()? })
    }

    field create_event(
        &executor,
        event: EventRequest,
        token: String
    ) -> FieldResult<Tokenized<Event>> {
        let token = token.parse::<Token>()?;
        let database = &executor.context().conn;
        crate::authorization::validate(
            &database,
            &token,
            "event".to_string(),
            ActionType::Create,
            ActionModifier::Own
        )?;
        let event = crate::resolvers::event::create(
            &token.uid,
            EventInsert::from_request(event),
            database
        )?;
        Ok(Tokenized { payload: event, token: token.to_string()? })
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
        let database = &executor.context().conn;
        let user = crate::resolvers::user::create(user_request, database)?;
        let token = Token::new(&user.email).to_string()?;
        Ok(Tokenized { payload: user, token })
    }

});
