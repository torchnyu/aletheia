use super::RequestContext;
use crate::resolvers::{event, project, user};
use crate::types::Token;
use crate::types::{
    Event, EventInsert, EventRequest, LoginRequest, Project, ProjectRequest, Tokenized, User,
    UserRequest,
};
use juniper::FieldResult;

pub struct MutationRoot {}

graphql_object!(MutationRoot: RequestContext as "Mutation" |&self| {
    description: "The root mutation object of the schema"

    field login(
        &executor,
        email: String,
        password: String,
    ) -> FieldResult<Tokenized<User>>  {
        let email = email.to_ascii_lowercase();
        let credentials = LoginRequest {
            email, password
        };
        let conn = &executor.context().conn;
        let user = user::login(&credentials, conn)?;
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
        let conn = &executor.context().conn;
        let user = user::get_by_email(&token.uid, conn)?;
        let project = project::create(
            &user.email,
            project,
            conn
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
        let conn = &executor.context().conn;
        let user = user::get_by_email(&token.uid, conn)?;
        let event = event::create(
            &user.email,
            EventInsert::from_request(event),
            conn
        )?;
        Ok(Tokenized { payload: event, token: token_string })
    }

    field register(
        &executor,
        email: String,
        password: String
    ) -> FieldResult<Tokenized<User>> {
        let email = email.to_ascii_lowercase();
        let user_request = UserRequest {
            display_name: None,
            email,
            password,
        };

        let conn = &executor.context().conn;
        let user = user::create(user_request, conn)?;
        let token = Token::new(&user.email).to_string()?;
        Ok(Tokenized { payload: user, token })
    }

    field send_reset_password_email(
        &executor,
        email: String,
        domain: String
    ) -> FieldResult<&'static str> {
        let email = email.to_ascii_lowercase();
        crate::resolvers::user::send_reset_email(&email, &domain, &executor.context().conn)?;
        Ok("Sent password reset email")
    }

    field reset_password(
        &executor,
        email: String,
        password: String,
        key: String
    ) -> FieldResult<Tokenized<User>> {
        let email = email.to_ascii_lowercase();
        let user = crate::resolvers::user::reset_password(&email, &password, &key, &executor.context().conn)?;
        let token = Token::new(&user.email).to_string()?;
        Ok(Tokenized { payload: user, token })
    }
});
