use gdnative::derive::{FromVariant, ToVariant};
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};

use crate::{utils::Stringable, Result};

pub const SAVE_DIR: &str = "user://config.toml";
const DEFAULT_SCAN_DEPTH: i64 = 2;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub scan_paths: Vec<String>,
    pub max_scan_depth: i64,
    pub projects: Vec<Project>,
}

#[derive(Serialize, Deserialize, Debug, ToVariant, FromVariant)]
pub struct Project {
    pub name: String,
    pub path: String,

    pub missing: bool,
    pub grayed: bool,
}

impl Project {
    pub fn new<T: Stringable>(name: T, path: T) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),

            missing: false,
            grayed: false,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            scan_paths: vec![home::home_dir()
                .unwrap_or(PathBuf::from("/"))
                .to_str()
                .unwrap_or("/")
                .to_string()],
            max_scan_depth: DEFAULT_SCAN_DEPTH,
            projects: vec![],
        }
    }

    pub fn load<T: Stringable>(&mut self, path: T) -> Result<()> {
        let path = PathBuf::from(path.to_string());

        let contents = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return Err(Box::new(e)),
        };

        let config: Config = match toml::from_str(&contents) {
            Ok(v) => v,
            Err(e) => return Err(Box::new(e)),
        };

        self.scan_paths = config.scan_paths;
        self.projects = config.projects;

        Ok(())
    }

    pub fn write<T: Stringable>(&self, path: T) -> Result<()> {
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
