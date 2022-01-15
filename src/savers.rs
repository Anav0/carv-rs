use std::{fs::File, io::Write, path::Path};

use crate::models::Repo;

pub(crate) trait Saver {
    fn save(path: &Path, repos: &Vec<Repo>);
}

pub(crate) struct JsonSaver;

impl Saver for JsonSaver {
    fn save(file_path: &Path, repos: &Vec<Repo>) {
        let mut file = File::create(file_path).expect("Failed to create cache file");

        let json = serde_json::to_string(repos).expect("Failed to serialize repos");

        println!("Results saved as: {}", file_path.display());

        file.write_all(json.as_bytes())
            .expect("Failed to save results as json");
    }
}

pub(crate) struct TomlSaver;
impl Saver for TomlSaver {
    fn save(file_path: &Path, repos: &Vec<Repo>) {
        todo!();
    }
}
