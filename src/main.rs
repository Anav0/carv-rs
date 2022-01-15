use std::env;

use github::CachedGithub;
use models::{Format, Parameters};
use savers::{JsonSaver, Saver, TomlSaver};

mod github;
mod models;
mod savers;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let parameters = Parameters::from(env::args());

    let saver: &dyn Saver = match parameters.formats {
        Format::Json => &JsonSaver {},
        Format::Toml => &TomlSaver {},
    };

    let github = CachedGithub::new(saver);

    let repos = github.get_repos(&parameters);

    if !parameters.quiet {
        for repo in repos {
            println!("{}\n", repo);
        }
    }

    Ok(())
}
