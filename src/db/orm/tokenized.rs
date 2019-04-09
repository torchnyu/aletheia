use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tokenized<T> {
    pub payload: T,
    pub token: String,
}

