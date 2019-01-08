#[macro_use]
extern crate failure_derive;

use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

mod github;
mod types;

use crate::types::{AletheiaError, Result, RulesConfig};

fn main() -> Result<()> {
    let repos: Vec<&'static str> = vec![
        "hacknyu/hacknyu-2019",
        "nicholaslyang/saber",
        "jsonkao/jasonkao.me",
    ];
    let config = load_config()?;
    let issues = github::check_repos(repos.as_slice(), config.into_rules()?)?;
    println!("{:?}", issues);
    Ok(())
}

fn read_config_file() -> Result<String> {
    let mut file = File::open("config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn load_config() -> Result<RulesConfig> {
    let contents = read_config_file()?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}
