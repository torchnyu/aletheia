use crate::utils::Result;
use image::FilterType;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

struct Format {
    name: &'static str,
    x_height: u32,
    y_height: u32,
}

static FORMATS: &'static [Format] = &[
    Format {
        name: "thumbnail",
        x_height: 120,
        y_height: 120,
    },
    Format {
        name: "medium",
        x_height: 480,
        y_height: 480,
    },
    Format {
        name: "large",
        x_height: 1080,
        y_height: 1080,
    },
];

pub fn call(original_file: &PathBuf, file_ext: &str) -> Result<Vec<PathBuf>> {
    let parent = original_file.parent().unwrap();
    let mut image_file = BufReader::new(File::open(original_file)?);
    let format = image::guess_format(image_file.fill_buf()?)?;
    let image = image::load(image_file, format)?;

    let mut file_names = Vec::new();
    for format in FORMATS {
        let file = image.resize(format.x_height, format.y_height, FilterType::Lanczos3);
        let file_path = parent.join(format!("{}.{}", format.name, file_ext));
        file.save(&file_path)?;
        file_names.push(file_path);
    }
    Ok(file_names)
}
