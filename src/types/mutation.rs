use super::{
    Context, LoginRequest, LoginResponse, Project, ProjectInsert, ProjectRequest, Tokenized,
};
use crate::tokens::Claims;
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
        let token = crate::tokens::create_token(&user.email)?;
        Ok(LoginResponse { user, token })
    }

    field create_project(
        &executor,
        name: String,
        repository_url: String,
        color: String,
        description: Option<String>,
        token: String
    ) -> FieldResult<Tokenized<Project>> {
        let token = Claims::from_string(token)?;
        let new_token = token.validate()?;
        let request = ProjectRequest { name, repository_url, color, description};
        let database = &executor.context().database;
        let project = crate::resolvers::project::insert(ProjectInsert::from_request(request), database)?;
        Ok(Tokenized { inner: project, token: new_token.to_string()? })
    }
});
