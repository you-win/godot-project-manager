use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};

use crate::utils::Stringable;

pub const SAVE_DIR: &str = "user://config.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    search_path: String,
    projects: Vec<Project>,
}

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            search_path: String::from("user://"),
            projects: vec![],
        }
    }

    pub fn load<T: Stringable>(&mut self, path: T) -> Result<(), Box<dyn std::error::Error>> {
        let path = PathBuf::from(path.to_string());

        let contents = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return Err(Box::new(e)),
        };

        let config: Config = match toml::from_str(&contents) {
            Ok(v) => v,
            Err(e) => return Err(Box::new(e)),
        };

        self.search_path = config.search_path;
        self.projects = config.projects;

        Ok(())
    }

    pub fn write<T: Stringable>(&self, path: T) -> Result<(), Box<dyn std::error::Error>> {
        let toml = match toml::to_string_pretty(self) {
            Ok(s) => s,
            Err(e) => return Err(Box::new(e)),
        };

        let path = PathBuf::from(path.to_string());

        let mut file = fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        match write!(&mut file, "{}", toml) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
