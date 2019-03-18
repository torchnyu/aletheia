use super::{
    Context, LoginRequest, LoginResponse, Project, ProjectInsert, ProjectRequest, Token, Tokenized,
};

use crate::sql_types::*;
use juniper::FieldResult;

pub struct MutationRoot {}

graphql_object!(MutationRoot: Context as "Mutation" |&self| {
    description: "The root mutation object of the schema"

    field login(
        &executor,
        email: String,
        password: String,
    ) -> FieldResult<LoginResponse>  {
        let database = &executor.context().database;
        let credentials = LoginRequest {
            email, password
        };
        let user = crate::resolvers::user::login(&credentials, &database)?;
        let token = Token::new(&user.email).to_string()?;
        Ok(LoginResponse { user, token })
    }

    field create_project(
        &executor,
        name: String,
        repository_url: String,
        color: String,
        description: Option<String>,
        token: String,
    ) -> FieldResult<Tokenized<Project>> {
        let token = token.parse::<Token>()?.validate()?;
        let database = &executor.context().database;
        let user = crate::resolvers::user::get_by_email(&token.uid, &database)?;
        crate::authorization::validate(
            &database,
            &user,
            "Project".to_string(),
            ActionType::Create,
            ActionModifier::Self_
        )?;
        let request = ProjectRequest { name, repository_url, color, description};
        let project = crate::resolvers::project::create(&token, ProjectInsert::from_request(request), database)?;
        Ok(Tokenized { payload: project, token: token.to_string()? })
    }
});
