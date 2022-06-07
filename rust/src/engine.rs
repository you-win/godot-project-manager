use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use gdnative::api::ProjectSettings;

use crate::{
    config::{Config, Project, SAVE_DIR},
    utils::{update_status, Stringable},
    Result,
};

const GODOT_PROJECT_FILE_EXTENSION: &str = "godot";
const GODOT_PROJECT_NAME_KEY: &str = "config/name";

const CANNOT_GET_PATH_MESSAGE: &str = "[failed to get path]";

/// The engine for the project manager. Depends on Godot singletons being available
pub struct Engine {
    config: Config,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            config: Config::new(),
        }
    }

    pub fn load_config(&mut self) -> Result<()> {
        let path = ProjectSettings::godot_singleton().globalize_path(SAVE_DIR);

        self.config.load(path.to_string())
    }

    pub fn write_config(&mut self) -> Result<()> {
        let path = ProjectSettings::godot_singleton().globalize_path(SAVE_DIR);

        self.config.write(path.to_string())
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    pub fn copy_directory<T: Stringable>(from: T, to: T) -> Result<()> {
        let from = PathBuf::from(from.to_string());
        let to = PathBuf::from(to.to_string());

        let mut options = fs_extra::dir::CopyOptions::new();
        options.copy_inside = true;
        options.overwrite = true;

        match fs_extra::dir::copy(from, to, &options) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn remove_directory<T: Stringable>(dir: T) -> Result<()> {
        let path = PathBuf::from(dir.to_string());

        match fs_extra::dir::remove(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn scan_for_projects(&self) -> Vec<Project> {
        let mut output = vec![];

        for path in self.config.scan_paths.iter() {
            let paths = match std::fs::read_dir(path) {
                Ok(p) => p,
                Err(e) => {
                    update_status(format!("Unable to read dir {}\n{}", path, e));
                    continue;
                }
            };

            for entry in paths {
                let entry = match &entry {
                    Ok(de) => de,
                    Err(e) => {
                        update_status(format!("Unable to handle entry {:?}\n{}", entry, e));
                        continue;
                    }
                };

                match &entry.file_type() {
                    Ok(ft) => {
                        if ft.is_file() || ft.is_symlink() {
                            continue;
                        }
                    }
                    Err(e) => {
                        update_status(format!(
                            "Unable to determine file type for {:?}\n{}",
                            &entry, e
                        ));
                        continue;
                    }
                }

                let path = &entry.path();
                let path = path.as_path();

                match scan_path_for_project_file(path) {
                    Some(project) => output.push(project),
                    None => update_status(format!("No project found at {}", path.display())),
                }
            }
        }

        output
    }
}

fn scan_path_for_project_file(project_path: &Path) -> Option<Project> {
    let files = match std::fs::read_dir(project_path) {
        Ok(f) => f,
        Err(e) => {
            update_status(format!(
                "Unable to read dir {}\n{}",
                project_path.display(),
                e
            ));
            return None;
        }
    };

    for file in files {
        let file = match &file {
            Ok(f) => f,
            Err(e) => {
                update_status(format!(
                    "Unable to get file information for {:?}\n{}",
                    file, e
                ));
                continue;
            }
        };

        let path = file.path();
        let path = path.as_path();

        if path
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap_or("")
            != GODOT_PROJECT_FILE_EXTENSION
        {
            continue;
        }

        let contents = match std::fs::read_to_string(path) {
            Ok(f) => f,
            Err(e) => {
                update_status(format!(
                    "Unable to read project file {}\n{}",
                    path.display(),
                    e
                ));
                continue;
            }
        };
        let project_name = match contents
            .split("\n")
            .filter(|x| x.starts_with(GODOT_PROJECT_NAME_KEY))
            .collect::<Vec<&str>>()
            .first()
        {
            Some(n) => n.to_string(),
            None => {
                update_status(format!(
                    "Unable to find project name for {}",
                    path.display()
                ));
                continue;
            }
        };

        return Some(Project::new(
            project_name,
            project_path.to_str().unwrap().to_string(),
        ));
    }

    None
}
