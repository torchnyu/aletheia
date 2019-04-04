use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Medium {
    id: i32,
    raw_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct MediumRequest {
    pub name: String,
    pub file: String,
}
