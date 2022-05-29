use gdnative::api::ProjectSettings;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::{utils::Stringable, Error, ConfigError};

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

    pub fn load<T: Stringable>(&mut self, path: T) -> Result<(), Error> {
        let path = PathBuf::from(path.to_string());

        let contents = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return Err(Error::ConfigError(ConfigError::FileNotFound)),
        };

        let mut config: Config = match toml::from_str(&contents) {
            Ok(v) => v,
            Err(e) => return Err(Error::from(e)),
        };

        self.search_path = config.search_path;
        self.projects = config.projects;

        Ok(())
    }

    pub fn write(&self) -> Result<(), Error> {
        let toml = match toml::to_string_pretty(self) {
            Ok(s) => s,
            Err(e) => return Err(Error::from(e)),
        };

        Ok(())
    }
}
