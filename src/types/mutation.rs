use super::{Context, LoginRequest, LoginResponse, Project, ProjectInsert, ProjectRequest};
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
        Ok(LoginResponse {  user, token })
    }

    field create_project(
        &executor,
        name: String,
        repository_url: String,
        color: String,
        description: Option<String>
    ) -> FieldResult<Project> {
        let request = ProjectRequest { name, repository_url, color, description};
        let database = &executor.context().database;
        Ok(crate::resolvers::project::insert(ProjectInsert::from_request(request), database)?)
    }
});
