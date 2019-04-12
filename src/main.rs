#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate juniper;
extern crate base64;
extern crate dotenv;
extern crate heck;
extern crate jsonwebtoken as jwt;
extern crate juniper_rocket;
extern crate multipart;
extern crate r2d2;
extern crate rand;
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate slug;
#[macro_use]
extern crate diesel_derive_enum;

use crate::db::Connection;
use crate::graphql::Context;
use crate::utils::Result;
use rocket::response::content;
use rocket::*;
use rocket_cors::CorsOptions;

mod authorization;
mod db;
mod github;
mod graphql;
mod resolvers;
mod routes;
#[allow(unused_imports)]
mod types;
mod utils;

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
    let schema = graphql::create_schema();
    let context = Context { database };
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn handle_graphql_post(
    request: juniper_rocket::GraphQLRequest,
    database: Connection,
) -> juniper_rocket::GraphQLResponse {
    let schema = graphql::create_schema();
    let context = Context { database };
    request.execute(&schema, &context)
}

fn main() -> Result<()> {
    let default = CorsOptions::default();
    let cors = CorsOptions::to_cors(&default)?;
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
        .mount(
            "/submissions",
            routes![routes::submissions::index, routes::submissions::create],
        )
        .mount("/media", routes![routes::media::create])
        .mount(
            "/",
            routes![index, graphiql, handle_graphql_get, handle_graphql_post],
        )
        .attach(cors)
        .attach(Connection::fairing())
        .launch();
    Ok(())
}
