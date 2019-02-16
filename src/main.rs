#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate jsonwebtoken as jwt;
extern crate rand;

use itertools::Itertools;
use std::fs::File;
use std::io::Read;

mod controllers;
mod github;
mod models;
mod routes;
mod schema;
mod tokens;
mod types;

use crate::types::{DbConn, Result, RulesConfig};
use rocket::*;

#[get("/<username>/<repo_name>")]
fn validate_repo(username: String, repo_name: String) -> Result<String> {
    let config = load_config()?;
    let repo = format!("{}/{}", username, repo_name).to_string();
    let issues = github::check_repo(&repo, config.into_rules()?)?;
    Ok(issues.iter().map(ToString::to_string).join("\n"))
}

#[get("/")]
fn index() -> String {
    "Welcome to Aletheia, a hackathon cheating detector!".to_string()
}

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    rocket::ignite()
        .mount(
            "/projects",
            routes![routes::projects::index, routes::projects::create],
        )
        .mount(
            "/users",
            routes![
                routes::users::index,
                routes::users::create,
                routes::users::login
            ],
        )
        .mount("/", routes![index])
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
