use crate::utils::Result;
use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn upload_image(local_filename: &Path) -> Result<String> {
    let client = S3Client::new(Region::UsEast1);
    let dest_filename = "test".to_string();
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
            let result = client.put_object(req).sync().expect("Couldn't PUT object");
            println!("{:#?}", result);
        }
    };
    Ok(dest_filename)
}
