#[macro_use]
extern crate failure_derive;

use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

mod errors;
mod github;

use crate::errors::{AletheiaError, Result};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    rules: Rules,
}

#[derive(Serialize, Deserialize, Debug)]
struct Rules {
    start_date: String,
}

fn main() -> Result<()> {
    let repos: Vec<&'static str> = vec![
        "hacknyu/hacknyu-2019",
        "nicholaslyang/saber",
        "jsonkao/jasonkao.me",
    ];
    let config = load_config()?;
    let start_date = config
        .rules
        .start_date
        .parse::<DateTime<Utc>>()?
        .timestamp();
    github::check_repos(repos.as_slice(), start_date)?;
    Ok(())
}

fn read_config_file() -> Result<String> {
    let mut file = File::open("config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn load_config() -> Result<Config> {
    let contents = read_config_file()?;
    let config = toml::from_str(&contents)?;
    println!("{:?}", config);
    Ok(config)
}
