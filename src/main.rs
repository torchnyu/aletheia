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
mod schema;
mod types;

use crate::models::Project;
use crate::types::{InsertableProject, Result, RulesConfig};
use dotenv::dotenv;
use rocket::*;
use rocket_contrib::databases::diesel as other_diesel;
use rocket_contrib::json::Json;
use std::env;

#[database("postgres_logs")]
pub struct LogsDbConn(diesel::PgConnection);

#[get("/<username>/<repo_name>")]
fn validate_repo(username: String, repo_name: String) -> Result<String> {
    let config = load_config()?;
    let repo = format!("{}/{}", username, repo_name).to_string();
    let issues = github::check_repo(&repo, config.into_rules()?)?;
    Ok(issues.iter().map(ToString::to_string).join("\n"))
}

#[get("/")]
pub fn index(conn: LogsDbConn) -> Result<Json<Vec<Project>>> {
    Ok(Json(controllers::all(&conn)?))
}

#[post("/", format = "application/json", data = "<project>")]
pub fn create(conn: LogsDbConn, project: Json<InsertableProject>) -> Result<Json<Project>> {
    let project = project.into_inner();
    Ok(Json(controllers::insert(project, &conn)?))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![validate_repo])
        .mount("/projects", routes![index, create])
        .attach(LogsDbConn::fairing())
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
