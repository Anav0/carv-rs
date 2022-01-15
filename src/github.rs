use std::{fs, path::Path};

use crate::{
    models::{Parameters, Repo},
    savers::Saver,
};

pub(crate) struct CachedGithub<'a> {
    saver: &'a dyn Saver,
}

impl<'a> CachedGithub<'a> {
    pub fn new(saver: &'a dyn Saver) -> Self {
        Self { saver }
    }

    pub fn get_repos(&self, parameters: &Parameters) -> Vec<Repo> {
        let ext = self.saver.get_ext();
        let file_path_str = format!("./{}{}", parameters.username, ext);

        let file_path = Path::new(&file_path_str);
        if parameters.clear_cache && file_path.exists() {
            fs::remove_file(file_path).expect("Failed to remove cached file");
        }
        let repos: Vec<Repo> = match file_path.exists() {
            true => self.saver.read(file_path),
            false => {
                let mut repos = self.fetch_repos(&parameters);

                repos.sort();

                self.saver.save(file_path, &repos);

                repos
            }
        };

        repos
    }

    fn fetch_repos(&self, parameters: &Parameters) -> Vec<Repo> {
        let url = format!("http://api.github.com/users/{}/repos", parameters.username);
        let body = ureq::get(&url)
            .call()
            .unwrap()
            .into_string()
            .expect("Failed to fetch");

        serde_json::from_str(&body).expect("Failed to parse body")
    }
}
