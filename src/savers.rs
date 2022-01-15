use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::models::Repo;

pub(crate) trait Saver {
    fn save(&self, path: &Path, repos: &Vec<Repo>);
    fn read(&self, path: &Path) -> Vec<Repo>;
    fn get_ext(&self) -> String;
}

pub(crate) struct JsonSaver;

impl Saver for JsonSaver {
    fn save(&self, file_path: &Path, repos: &Vec<Repo>) {
        let mut file = File::create(file_path).expect("Failed to create cache file");

        let json = serde_json::to_string(repos).expect("Failed to serialize repos");

        println!("Results saved as: {}", file_path.display());

        file.write_all(json.as_bytes())
            .expect("Failed to save results as json");
    }

    fn get_ext(&self) -> String {
        String::from(".json")
    }

    fn read(&self, path: &Path) -> Vec<Repo> {
        let json = fs::read_to_string(path).unwrap();
        serde_json::from_str(&json).expect("Failed at parsing cached json for user")
    }
}

pub(crate) struct TomlSaver;
impl Saver for TomlSaver {
    fn save(&self, file_path: &Path, repos: &Vec<Repo>) {
        todo!();
    }
    fn read(&self, path: &Path) -> Vec<Repo> {
        todo!()
    }
    fn get_ext(&self) -> String {
        String::from(".toml")
    }
}
