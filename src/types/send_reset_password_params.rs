use serde_derive::{Deserialize, Serialize};

// The most useless struct ever...
#[derive(Serialize, Deserialize, GraphQLInputObject)]
pub struct SendResetPasswordParams {
    pub email: String,
    pub domain: String,
}
