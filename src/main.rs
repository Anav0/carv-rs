use std::{
    env,
    fs::{self},
    path::Path,
};

use models::{Format, Parameters, Repo};
use savers::{JsonSaver, Saver, TomlSaver};

mod models;
mod savers;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let parameters = Parameters::from(env::args());

    let file_path_str = match parameters.formats {
        Format::Json => {
            format!("./{}.json", parameters.username)
        }
        Format::Toml => {
            format!("./{}.toml", parameters.username)
        }
    };

    let file_path = Path::new(&file_path_str);

    if parameters.clear_cache && file_path.exists() {
        fs::remove_file(file_path).expect("Failed to remove cached file");
    }

    let repos: Vec<Repo> = match file_path.exists() {
        true => read_repos_from_cache(file_path),
        false => {
            let mut repos = fetch_repos_from_api(&parameters.username);

            repos.sort();

            match parameters.formats {
                Format::Json => JsonSaver::save(file_path, &repos),
                Format::Toml => TomlSaver::save(file_path, &repos),
            }

            repos
        }
    };

    if !parameters.quiet {
        for repo in repos {
            println!("{}", repo);
            println!();
        }
    }

    Ok(())
}

fn fetch_repos_from_api(username: &str) -> Vec<Repo> {
    let url = format!("http://api.github.com/users/{}/repos", username);

    let body = ureq::get(&url)
        .call()
        .unwrap()
        .into_string()
        .expect("Failed to fetch");

    serde_json::from_str(&body).expect("Failed to parse body")
}

fn read_repos_from_cache(file_path: &Path) -> Vec<Repo> {
    let json = fs::read_to_string(file_path).unwrap();
    serde_json::from_str(&json).expect("Failed at parsing cached json for user")
}
