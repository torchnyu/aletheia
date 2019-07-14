use crate::db::models::{Medium, MediumInsert};
use crate::db::schema::media;
use crate::db::PgConnection;
use crate::diesel::RunQueryDsl;
use crate::services::*;
use crate::utils::Result;
use std::path::Path;

pub fn all(conn: &PgConnection) -> Result<Vec<Medium>> {
    Ok(media::table.load::<Medium>(conn)?)
}

pub fn create(
    local_filename: &Path,
    file_ext: String,
    project_id: Option<i32>,
    user_id: Option<i32>,
    conn: &PgConnection,
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
        .get_result(conn)?)
}
