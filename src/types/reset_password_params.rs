use serde_derive::{Deserialize, Serialize};

// The (second) most useless struct ever...
#[derive(Serialize, Deserialize, GraphQLInputObject)]
pub struct ResetPasswordParams {
    pub email: String,
    pub password: String,
    pub key: String,
}
