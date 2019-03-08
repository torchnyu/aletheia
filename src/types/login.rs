use super::{Context, UserResponse};

pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

graphql_object!(LoginResponse: Context |&self| {

    field user(&executor) -> &UserResponse {
        &self.user
    }

    field token(&executor) -> &str {
        &self.token
    }
});
