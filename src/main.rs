#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate failure_derive;

use itertools::Itertools;
use std::fs::File;
use std::io::Read;

mod github;
mod types;

use crate::types::{Result, RulesConfig};
use rocket::*;

#[get("/<username>/<repo_name>")]
fn index(username: String, repo_name: String) -> Result<String> {
    let config = load_config()?;
    let repo = format!("{}/{}", username, repo_name).to_string();
    let issues = github::check_repo(&repo, config.into_rules()?)?;
    Ok(issues.iter().map(ToString::to_string).join("\n"))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
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
