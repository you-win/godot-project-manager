mod about_screen;
mod addons_screen;
mod config;
mod engine;
mod main_screen;
mod projects_screen;
mod settings_screen;
mod templates_screen;
mod utils;

use gdnative::{
    api::{LineEdit, TextEdit},
    prelude::*,
};
use lazy_static::lazy_static;
use std::sync::Mutex;

pub static mut STATUS: Option<Ref<LineEdit>> = None;
pub static mut LOGS: Option<Ref<TextEdit>> = None;
lazy_static! {
    pub static ref ENGINE: Mutex<engine::Engine> = Mutex::new(engine::Engine::new());
}

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

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn init(handle: InitHandle) {
    handle.add_class::<main_screen::MainScreen>();

    handle.add_class::<projects_screen::ProjectsScreen>();
    handle.add_class::<addons_screen::AddonsScreen>();
    handle.add_class::<templates_screen::TemplatesScreen>();
    handle.add_class::<settings_screen::SettingsScreen>();
    handle.add_class::<about_screen::AboutScreen>();
}

godot_init!(init);
