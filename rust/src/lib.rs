mod about_screen;
mod addons_screen;
mod config;
mod main_screen;
mod projects_screen;
mod settings_screen;
mod templates_screen;
mod utils;

use gdnative::prelude::*;

const APP_NAME: &str = "Godot Project Manager";
const VERSION_MAJOR: u16 = 0;
const VERSION_MINOR: u16 = 0;
const VERSION_PATCH: u16 = 1;

pub fn app_name_and_version() -> String {
    format!(
        "{} - {}.{}.{}",
        APP_NAME, VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH
    )
}

#[derive(Debug)]
pub enum Error {
    GenericError(String),
    LoadError(String),
    ConfigError(ConfigError),
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,
    Load(String),
    Write(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::GenericError(e.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::ConfigError(ConfigError::Load(e.to_string()))
    }
}

impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Error::ConfigError(ConfigError::Write(e.to_string()))
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<main_screen::MainScreen>();

    handle.add_class::<projects_screen::ProjectsScreen>();
    handle.add_class::<addons_screen::AddonsScreen>();
    handle.add_class::<templates_screen::TemplatesScreen>();
    handle.add_class::<settings_screen::SettingsScreen>();
    handle.add_class::<about_screen::AboutScreen>();
}

godot_init!(init);
