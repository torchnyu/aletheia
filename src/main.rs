#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]
#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

use itertools::Itertools;
use std::fs::File;
use std::io::Read;

mod controllers;
mod github;
mod models;
mod routes;
mod schema;
mod types;

use crate::types::{DbConn, Result, RulesConfig};
use rocket::*;
use rocket_contrib::json::Json;

#[get("/<username>/<repo_name>")]
fn validate_repo(username: String, repo_name: String) -> Result<String> {
    let config = load_config()?;
    let repo = format!("{}/{}", username, repo_name).to_string();
    let issues = github::check_repo(&repo, config.into_rules()?)?;
    Ok(issues.iter().map(ToString::to_string).join("\n"))
}

fn main() {
    rocket::ignite()
        .mount("/projects", routes![routes::index, routes::create])
        .attach(DbConn::fairing())
        .launch();
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
