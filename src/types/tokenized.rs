use super::{Context, Project};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tokenized<T> {
    pub payload: T,
    pub token: String,
}

graphql_object!(Tokenized<Project>: Context as "AuthenticatedProject" |&self| {
    field token() -> &str {
        &self.token
    }

    field project() -> &Project {
        &self.payload
    }
});
