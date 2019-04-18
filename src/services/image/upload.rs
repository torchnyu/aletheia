use crate::utils::Result;
use chrono::prelude::*;
use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn call(files: Vec<PathBuf>) -> Result<String> {
    let datetime = Utc::now();
    let date_string = format!("{}", datetime.format("%Y-%m-%d-%H%M%S"));
    let folder_name = Path::new(&date_string);
    let client = S3Client::new(Region::UsEast1);
    let bucket = env::var("BUCKET_NAME")?;
    for file in files {
        let file_name = file.file_name().unwrap();
        let file_path = folder_name.join(file_name);
        let mut f = File::open(&file)?;
        let mut contents: Vec<u8> = Vec::new();
        match f.read_to_end(&mut contents) {
            Err(err) => panic!("Error opening file to send to S3: {}", err),
            Ok(_) => {
                let req = PutObjectRequest {
                    bucket: bucket.to_owned(),
                    key: file_path.to_string_lossy().to_string(),
                    body: Some(contents.into()),
                    ..Default::default()
                };
                client.put_object(req).sync()?;
            }
        };
    }
    Ok(date_string)
}
