use std::{cmp::Ordering, env::Args, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub(crate) enum Format {
    Json,
    Toml,
}

#[derive(Debug)]
pub(crate) struct Parameters {
    pub clear_cache: bool,
    pub username: String,
    pub formats: Format,
    pub quiet: bool,
}

impl From<Args> for Parameters {
    fn from(args: Args) -> Self {
        let mut parameters = Parameters::default();
        let args: Vec<String> = args.collect();

        let mut i = 0;
        for arg in &args {
            match arg.as_str() {
                "-c" => parameters.clear_cache = true,
                "-u" => parameters.username = args[i + 1].clone(),
                "-f" => {
                    parameters.quiet = true;
                    let format_str = args[i + 1].as_str();
                    parameters.formats = match format_str {
                        "json" => Format::Json,
                        "toml" => Format::Toml,
                        _ => panic!("Picked unsupported output file format (json | toml)"),
                    }
                }
                _ => {}
            }
            i += 1;
        }

        parameters
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            clear_cache: false,
            username: String::new(),
            formats: Format::Json,
            quiet: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Repo {
    pub id: usize,
    pub url: String,
    pub name: String,
    pub description: Option<String>,
    pub stargazers_count: usize,
}

impl Display for Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match &self.description {
            Some(x) => format!("desc: {}", x),
            None => String::from(""),
        };
        write!(
            f,
            "stars: {}\nname: {}\n{}",
            self.stargazers_count, self.name, desc
        )
    }
}

impl Ord for Repo {
    fn cmp(&self, other: &Self) -> Ordering {
        (&other.stargazers_count).cmp(&self.stargazers_count)
    }
}
impl PartialOrd for Repo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((&other.stargazers_count).cmp(&self.stargazers_count))
    }
}
impl PartialEq for Repo {
    fn eq(&self, other: &Self) -> bool {
        &self.stargazers_count == &other.stargazers_count
    }
}
impl Eq for Repo {}
