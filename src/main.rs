extern crate reqwest;
extern crate serde_json;

use reqwest::Client;
use serde_json::{Error, Value};

static REPOS: &'static [&'static str] = &[
    "hacknyu/hacknyu-2019",
    "nicholaslyang/saber",
    "jsonkao/jasonkao.me",
];

fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    for repo in REPOS {
        let url = format!("https://api.github.com/repos/{}", repo);
        let body = client.get(&url).send()?.text()?;
        let parsedBody: Result<Value, Error> = serde_json::from_str(&body);
        if let Ok(v) = parsedBody {
            println!("{}", v["created_at"]);
        }
    }

    Ok(())
}
