use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

use models::{Format, Parameters, Repo};

mod models;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut parameters = Parameters::new();
    let args: Vec<String> = env::args().collect();

    let mut i = 0;
    for arg in &args {
        match arg.as_str() {
            "-c" => parameters.clear_cache = true,
            "-u" => parameters.username = &args[i + 1],
            "-f" => {
                let format_str = args[i + 1].as_str();
                parameters.formats = match format_str {
                    "j" => Format::Json,
                    "t" => Format::Toml,
                    _ => panic!("Picked unsupported output file format ( j | t)"),
                }
            }
            _ => {}
        }
        i += 1;
    }

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
            let mut repos = fetch_repos_from_api(parameters.username);

            repos.sort();

            match parameters.formats {
                Format::Json => save_json(file_path, &repos),
                Format::Toml => save_toml(file_path, &repos),
            }

            repos
        }
    };

    for repo in repos {
        println!("{}", repo);
        println!();
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

fn save_json(file_path: &Path, repos: &Vec<Repo>) {
    let mut file = File::create(file_path).expect("Failed to create cache file");

    let json = serde_json::to_string(repos).expect("Failed to serialize repos");

    println!("results saved as; {}", file_path.display());

    file.write_all(json.as_bytes())
        .expect("Failed to save results as json");
}

fn save_toml(file_path: &Path, repos: &Vec<Repo>) {
    todo!()
}
