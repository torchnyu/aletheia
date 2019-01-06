extern crate reqwest;
extern crate serde_json;
extern crate toml;

use reqwest::Client;
use serde_json::Error;
use std::fs::File;
use std::io::Read;
use toml::Value;

static REPOS: &'static [&'static str] = &[
    "hacknyu/hacknyu-2019",
    "nicholaslyang/saber",
    "jsonkao/jasonkao.me",
];

fn main() -> Result<(), reqwest::Error> {
    if let Ok(contents) = read_config_file() {
        if let Ok(config) = parse_config(contents) {
            match config {
                Value::Table(t) => println!("{:?}", t["rules"]),
                _ => println!("WRONG TYPE"),
            }
        }
    }
    let client = Client::new();
    for repo in REPOS {
        let url = format!("https://api.github.com/repos/{}", repo);
        let body = client.get(&url).send()?.text()?;
        let parsed_body: Result<serde_json::Value, Error> = serde_json::from_str(&body);
        if let Ok(v) = parsed_body {
            println!("{}", v["created_at"]);
        }
    }

    Ok(())
}

fn read_config_file() -> std::io::Result<String> {
    let mut file = File::open("config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_config(contents: String) -> Result<Value, toml::de::Error> {
    contents.parse::<toml::value::Value>()
}
