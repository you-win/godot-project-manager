use gdnative::api::ProjectSettings;

use crate::config::{Config, SAVE_DIR};

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

    pub fn load_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let path = ProjectSettings::godot_singleton().globalize_path(SAVE_DIR);

        self.config.load(path.to_string())
    }

    pub fn write_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let path = ProjectSettings::godot_singleton().globalize_path(SAVE_DIR);

        self.config.write(path.to_string())
    }
}
