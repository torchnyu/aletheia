use super::{Context, User};

pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

graphql_object!(LoginResponse: Context |&self| {

    field user(&executor) -> &User {
        &self.user
    }

    field token(&executor) -> &str {
        &self.token
    }
});
