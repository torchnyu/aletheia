#[macro_use]
extern crate failure_derive;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use toml::Value;

mod errors;
mod github;

use crate::errors::{AletheiaError, Result};

fn main() -> Result<()> {
    let repos: Vec<&'static str> = vec![
        "hacknyu/hacknyu-2019",
        "nicholaslyang/saber",
        "jsonkao/jasonkao.me",
    ];
    github::check_repos(repos.as_slice());
    let config = load_config()?;
    Ok(())
}

fn read_config_file() -> Result<String> {
    let mut file = File::open("config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_config(contents: String) -> Result<Value> {
    Ok(contents.parse::<toml::value::Value>()?)
}

fn load_config() -> Result<BTreeMap<String, Value>> {
    let contents = read_config_file()?;
    let config = parse_config(contents)?;
    match config {
        Value::Table(t) => Ok(t),
        _ => Err(AletheiaError::ConfigError {
            message: "No table".to_string(),
        })?,
    }
}
