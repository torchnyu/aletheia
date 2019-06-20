use crate::db::connection::DatabaseContext;
use crate::db::models::{Medium, MediumInsert};
use crate::db::schema::media;
use crate::diesel::RunQueryDsl;
use crate::services::*;
use crate::utils::Result;
use std::path::Path;

struct MediaResolver {
    conn: diesel::PgConnection
};

impl MediaResolver {

    pub fn all(&self) -> Result<Vec<Medium>> {
        Ok(media::table.load::<Medium>(self.conn)?)
    }

    pub fn create(
        &self,
        local_filename: &Path,
        file_ext: String,
        project_id: Option<i32>,
        user_id: Option<i32>,
    ) -> Result<Medium> {
        let file_names = image::resize(&local_filename.to_path_buf(), &file_ext)?;
        let folder_name = image::upload(file_names)?;
        let medium = MediumInsert {
            folder_name,
            user_id,
            project_id,
            file_ext,
        };
        Ok(diesel::insert_into(media::table)
           .values(&medium)
           .get_result(self.conn)?)
    }

}
