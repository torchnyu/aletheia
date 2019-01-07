extern crate failure;
extern crate reqwest;
extern crate serde_json;
extern crate toml;
#[macro_use]
extern crate failure_derive;

use reqwest::Client;
use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::io::Read;
use toml::Value;

use failure::err_msg;
use failure::Error;

#[derive(Debug, Fail)]
enum AletheiaError {
    #[fail(display = "Config error: {}", message)]
    ConfigError { message: String },
}

pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let repos: Vec<&'static str> = vec![
        "hacknyu/hacknyu-2019",
        "nicholaslyang/saber",
        "jsonkao/jasonkao.me",
    ];
    check_repos(repos.as_slice());
    let config = load_config()?;
    println!("{:?}", config);
    Ok(())
}

fn check_repos(repos: &[&'static str]) -> Result<()> {
    let client = Client::new();
    for repo in repos {
        let url = format!("https://api.github.com/repos/{}", repo);
        let body = client.get(&url).send()?.text()?;
        let parsed_body: serde_json::Value = serde_json::from_str(&body)?;
        println!("{}", parsed_body["created_at"]);
    }
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
