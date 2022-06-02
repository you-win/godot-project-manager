use std::{ffi::OsStr, path::PathBuf};

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

    pub fn scan_for_projects(&self) -> Option<Vec<Project>> {
        let mut output = vec![];

        for path in self.config.scan_paths.iter() {
            // TODO
            update_status(path);

            for entry in walkdir::WalkDir::new(path)
                .min_depth(1)
                .max_depth(self.config.max_scan_depth.try_into().unwrap_or_else(|_| {
                    update_status("Unable to read max_scan_depth from config with error");
                    10
                }))
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_file() {
                    continue;
                }

                let path = entry.path();

                let extension = path
                    .extension()
                    .unwrap_or(OsStr::new(""))
                    .to_str()
                    .unwrap_or("");

                if extension != GODOT_PROJECT_FILE_EXTENSION {
                    // TODO
                    update_status(extension);
                    continue;
                }

                let file_contents = match std::fs::read_to_string(&path) {
                    Ok(f) => f,
                    Err(e) => {
                        update_status(format!(
                            "Unable to read project file: {}\n{}",
                            path.to_str().unwrap_or(CANNOT_GET_PATH_MESSAGE),
                            e
                        ));
                        continue;
                    }
                };
                let project_name = match file_contents
                    .split("\n")
                    .filter(|x| x.starts_with(GODOT_PROJECT_NAME_KEY))
                    .collect::<Vec<&str>>()
                    .first()
                {
                    Some(n) => n.to_string(),
                    None => {
                        update_status(format!(
                            "Unable to find project name for {}",
                            path.to_str().unwrap_or(CANNOT_GET_PATH_MESSAGE)
                        ));
                        continue;
                    }
                };

                let parent = path.parent();
                if parent.is_none() {
                    update_status(format!(
                        "Could not find parent directory for {}",
                        path.to_str().unwrap_or(CANNOT_GET_PATH_MESSAGE)
                    ));
                    continue;
                }

                let parent = match parent.unwrap().to_str() {
                    Some(p) => p.to_string(),
                    None => {
                        update_status(format!(
                            "Unable to get path for Godot project at {}",
                            path.to_str().unwrap_or(CANNOT_GET_PATH_MESSAGE)
                        ));
                        continue;
                    }
                };

                output.push(Project::new(&project_name, &parent));
            }
        }

        update_status(format!("{:?}", output));

        None
    }
}
