use crate::db::schema::media;
use crate::utils::Result;
use diesel::{self, AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::env;

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "media"]
pub struct Medium {
    pub id: i32,
    pub folder_name: String,
    pub project_id: Option<i32>,
    pub user_id: Option<i32>,
    pub file_ext: String,
}

#[derive(Insertable)]
#[table_name = "media"]
pub struct MediumInsert {
    pub folder_name: String,
    pub project_id: Option<i32>,
    pub user_id: Option<i32>,
    pub file_ext: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediumResponse {
    pub id: i32,
    pub folder_name: String,
    pub project_id: Option<i32>,
    pub file_ext: String,
    pub large_url: String,
    pub medium_url: String,
    pub thumbnail_url: String,
}

impl TryFrom<Medium> for MediumResponse {
    type Error = failure::Error;

    fn try_from(m: Medium) -> Result<Self> {
        let large_url = m.large_url()?;
        let medium_url = m.medium_url()?;
        let thumbnail_url = m.thumbnail_url()?;
        Ok(MediumResponse {
            id: m.id,
            folder_name: m.folder_name,
            project_id: m.project_id,
            file_ext: m.file_ext,
            large_url,
            medium_url,
            thumbnail_url,
        })
    }
}

impl Medium {
    fn generate_url(&self, size: &'static str) -> Result<String> {
        let bucket_name = env::var("BUCKET_NAME")?;
        Ok(format!(
            "https://{}.s3.amazonaws.com/{}/{}.{}",
            bucket_name, self.folder_name, size, self.file_ext
        ))
    }

    pub fn large_url(&self) -> Result<String> {
        self.generate_url("large")
    }

    pub fn medium_url(&self) -> Result<String> {
        self.generate_url("medium")
    }

    pub fn thumbnail_url(&self) -> Result<String> {
        self.generate_url("thumbnail")
    }
}
