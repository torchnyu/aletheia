//use crate::utils::Result;
use multipart::server::save::SaveDir;
use rocket::post;
//use slug::slugify;

use multipart::server::save::Entries;
use multipart::server::save::SaveResult::*;
use multipart::server::Multipart;

use rocket::http::{ContentType, Status};
use rocket::response::status::Custom;
use rocket::Data;

static MAX_BYTES: u64 = 128_000_000;

#[post("/", data = "<data>")]
pub fn create(
    content_type: &ContentType,
    data: Data,
) -> core::result::Result<String, Custom<String>> {
    if !content_type.is_form_data() {
        Err(Custom(
            Status::BadRequest,
            "Content-Type not multipart/form-data".into(),
        ))?;
    }

    let (_, boundary) = content_type
        .params()
        .find(|&(key, _)| key == "boundary")
        .ok_or_else(|| {
            Custom(
                Status::BadRequest,
                "`Content-Type: multipart/form-data` boundary param not provided".into(),
            )
        })?;

    match process_file_upload(boundary, data) {
        Ok(_) => Ok("Processed correctly".into()),
        Err(err) => Err(err),
    }
}

fn process_file_upload(boundary: &str, data: Data) -> core::result::Result<(), Custom<String>> {
    match Multipart::with_body(data.open(), boundary)
        .save()
        .size_limit(MAX_BYTES)
        .with_dir("./tmp")
    {
        Full(entries) => Ok(process_entries(entries)),
        Partial(partial, reason) => {
            let mut err_msg = format!("Request partially processed: {:?}", reason);
            if let Some(field) = partial.partial {
                err_msg = format!("{}\n Stopped on field: {:?}", err_msg, field.source.headers);
            }
            Err(Custom(Status::InternalServerError, err_msg))
        }
        Error(e) => Err(Custom(
            Status::InternalServerError,
            format!("Failed to save file: {:?}", e),
        )),
    }
}

fn process_entries(entries: Entries) {
    match entries.save_dir {
        SaveDir::Temp(temp) => println!("SAVE DIR TEMP: {}", temp.path().display()),
        SaveDir::Perm(path_buf) => println!(
            "SAVE DIR PERM: {}",
            path_buf.to_str().unwrap_or("NONE".into())
        ),
    }

    for entry in entries.fields.iter() {
        println!("{:?}", entry);
    }
}
