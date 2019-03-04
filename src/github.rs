// use crate::types::{Issue, Repository, Result, Rules};
// use reqwest::Client;

// static GITHUB_API: &'static str = "https://api.github.com/repos/";

// pub fn check_repo(repo_name: &String, rules: Rules) -> Result<Vec<Issue>> {
//     let client = Client::new();
//     let mut issues = Vec::new();
//     let url = format!("{}{}", GITHUB_API, repo_name);
//     let body = client.get(&url).send()?.text()?;
//     let repo: Repository = Repository::new(serde_json::from_str(&body)?)?;
//     if rules.start_date.timestamp() > repo.created_at.timestamp() {
//         issues.push(Issue::RepoCreationDate {
//             start_date: rules.start_date,
//             repo_creation_date: repo.created_at,
//         });
//     }
//     if repo.forks_count > rules.max_collaborators {
//         issues.push(Issue::TeamSize {
//             max_collaborators: rules.max_collaborators,
//             collaborators: repo.forks_count,
//         });
//     }
//     Ok(issues)
// }

// #[get("/<username>/<repo_name>")]
// fn validate_repo(username: String, repo_name: String) -> Result<String> {
//     let config = load_config()?;
//     let repo = format!("{}/{}", username, repo_name).to_string();
//     let issues = github::check_repo(&repo, config.into_rules()?)?;
//     Ok(issues.iter().map(ToString::to_string).join("\n"))
// }

// fn read_config_file() -> Result<String> {
//     let mut file = File::open("config.toml")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }

// fn load_config() -> Result<RulesConfig> {
//     let contents = read_config_file()?;
//     let config = toml::from_str(&contents)?;
//     Ok(config)
// }
