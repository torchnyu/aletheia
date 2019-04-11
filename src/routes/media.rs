//use crate::utils::Result;
use multipart::server::save::SaveDir;
use rocket::post;
//use slug::slugify;

use multipart::server::save::Entries;
use multipart::server::save::SaveResult::*;
use multipart::server::save::SavedData;
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

    process_file_upload(boundary, data)
}

fn process_file_upload(boundary: &str, data: Data) -> core::result::Result<String, Custom<String>> {
    match Multipart::with_body(data.open(), boundary)
        .save()
        .size_limit(MAX_BYTES)
        .temp()
    {
        Full(entries) => process_entries(entries),
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

fn process_entries(entries: Entries) -> core::result::Result<String, Custom<String>> {
    match entries.fields.get("file") {
        Some(field) => match &field[0].data {
            SavedData::File(path, _) => {
                match crate::resolvers::medium::upload_image(path.as_path()) {
                    Ok(s) => Ok(s),
                    Err(_) => Err(Custom(
                        Status::InternalServerError,
                        "Failed to upload file".to_string(),
                    )),
                }
            }
            _ => Err(Custom(
                Status::InternalServerError,
                "Internal error, please check server logs for details".to_string(),
            )),
        },
        None => Err(Custom(
            Status::InternalServerError,
            format!("No `file` given!"),
        )),
    }
}
