use rocket::post;

use crate::db::Connection;
use crate::types::Medium;
use multipart::server::save::Entries;
use multipart::server::save::SaveResult::*;
use multipart::server::save::SavedData;
use multipart::server::Multipart;
use std::ffi::OsStr;
use std::path::Path;

use rocket::http::{ContentType, Status};
use rocket::response::status::Custom;
use rocket::Data;
use rocket_contrib::json::Json;

static MAX_BYTES: u64 = 128_000_000;

#[post("/", data = "<data>")]
pub fn create(
    conn: Connection,
    content_type: &ContentType,
    data: Data,
) -> core::result::Result<Json<Medium>, Custom<String>> {
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

    let entries = process_file_upload(boundary, data)?;
    Ok(Json(process_entries(entries, conn)?))
}

fn process_file_upload(
    boundary: &str,
    data: Data,
) -> core::result::Result<Entries, Custom<String>> {
    match Multipart::with_body(data.open(), boundary)
        .save()
        .size_limit(MAX_BYTES)
        .temp()
    {
        Full(entries) => Ok(entries),
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

fn process_entries(
    entries: Entries,
    conn: Connection,
) -> core::result::Result<Medium, Custom<String>> {
    let field = match entries.fields.get("file") {
        Some(field) => field,
        None => {
            return Err(Custom(
                Status::InternalServerError,
                format!("No `file` given!"),
            ))
        }
    };
    let file_ext = if let Some(filename) = &(field[0].headers.filename) {
        match Path::new(filename).extension().and_then(OsStr::to_str) {
            Some(ext) => ext,
            None => {
                return Err(Custom(
                    Status::BadRequest,
                    "Invalid file extension".to_string(),
                ))
            }
        }
    } else {
        return Err(Custom(Status::BadRequest, "No filename given!".to_string()));
    };
    match &field[0].data {
        SavedData::File(path, _) => {
            match crate::resolvers::medium::create(path.as_path(), file_ext.to_owned(), &conn) {
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
    }
}
