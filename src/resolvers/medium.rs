use crate::db::connection::DatabaseContext;
use crate::db::models::{Medium, MediumInsert};
use crate::db::schema::media;
use crate::diesel::RunQueryDsl;
use crate::services::*;
use crate::utils::Result;
use std::path::Path;

pub fn all(db: &DatabaseContext) -> Result<Vec<Medium>> {
    Ok(media::table.load::<Medium>(db.conn)?)
}

pub fn create(
    local_filename: &Path,
    file_ext: String,
    project_id: Option<i32>,
    user_id: Option<i32>,
    db: &DatabaseContext,
) -> Result<Medium> {
    let file_names = image::resize(&local_filename.to_path_buf(), &file_ext)?;
    let folder_name = image::upload(file_names)?;
    let medium = MediumInsert {
        folder_name,
        user_id,
        project_id,
    };
    Ok(diesel::insert_into(media::table)
        .values(&medium)
        .get_result(db.conn)?)
}
