#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate juniper;
extern crate dotenv;
extern crate jsonwebtoken as jwt;
extern crate juniper_rocket;
extern crate rand;

use crate::db::Connection;
use itertools::Itertools;
use juniper::{EmptyMutation, RootNode};
use rocket::response::content;
use rocket::State;
use rocket::*;
use std::fs::File;
use std::io::Read;

mod controllers;
mod db;
mod github;
mod models;
mod routes;
mod schema;
mod tokens;
mod types;

type Schema = RootNode<'static, Connection, EmptyMutation<Connection>>;

#[get("/")]
fn index() -> String {
    "Welcome to Aletheia, HackNYU's centralized API!".to_string()
}

#[get["/graphiql"]]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphiql")
}

#[get("/graphql?<request>")]
fn handle_graphql_get(
    request: juniper_rocket::GraphQLRequest,
    database: Connection,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
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
        .mount("/", routes![index, graphiql])
        .manage(db::init_pool())
        .launch();
}
