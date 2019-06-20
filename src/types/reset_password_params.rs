use serde_derive::{Deserialize, Serialize};

// The most useless struct ever...
#[derive(Serialize, Deserialize)]
pub struct ResetPasswordParams {
    pub email: String
}
