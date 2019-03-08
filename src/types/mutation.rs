use super::{Context, LoginRequest, LoginResponse};
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
        let user = crate::controllers::users_controller::login(&credentials, &database)?;
        let token = crate::tokens::create_token(&user.email)?;
        Ok(LoginResponse {  user, token })
    }
});
