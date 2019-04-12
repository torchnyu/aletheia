use crate::db::models::{Medium, MediumInsert};
use crate::db::schema::media;
use crate::diesel::RunQueryDsl;
use crate::utils::Result;
use chrono::prelude::*;
use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn create(
    local_filename: &Path,
    file_ext: String,
    conn: &diesel::PgConnection,
) -> Result<Medium> {
    let datetime = Utc::now();
    let dest_filename = format!("{}.{}", datetime.format("%Y-%m-%d-%H%M%S"), file_ext);
    let client = S3Client::new(Region::UsEast1);
    let bucket = env::var("BUCKET_NAME")?;
    let mut f = File::open(local_filename)?;
    let mut contents: Vec<u8> = Vec::new();
    match f.read_to_end(&mut contents) {
        Err(err) => panic!("Error opening file to send to S3: {}", err),
        Ok(_) => {
            let req = PutObjectRequest {
                bucket: bucket.to_owned(),
                key: dest_filename.to_owned(),
                body: Some(contents.into()),
                ..Default::default()
            };
            client.put_object(req).sync()?;
        }
    };
    let medium = MediumInsert {
        file_name: dest_filename,
        project_id: None,
    };
    Ok(diesel::insert_into(media::table)
        .values(&medium)
        .get_result(conn)?)
}
