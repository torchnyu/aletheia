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
        project: ProjectRequest,
        token: String
    ) -> FieldResult<Tokenized<Project>> {
        let token = token.parse::<Token>()?;
        let database = &executor.context().database;
        crate::authorization::validate(
            &database,
            &token,
            Type::Project,
            ActionType::Create,
            ActionModifier::Own
        )?;
        let project = crate::resolvers::project::create(
            &token,
            ProjectInsert::from_request(project), database
        )?;
        Ok(Tokenized { payload: project, token: token.to_string()? })
    }

});
