use std::{cmp::Ordering, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub(crate) enum Format {
    Json,
    Toml,
}

#[derive(Debug)]
pub(crate) struct Parameters<'a> {
    pub clear_cache: bool,
    pub username: &'a str,
    pub formats: Vec<Format>,
}

impl<'a> Parameters<'a> {
    pub fn new() -> Self {
        Self {
            clear_cache: false,
            username: "",
            formats: vec![],
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
