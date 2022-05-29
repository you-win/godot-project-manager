use gdnative::api::{LineEdit, TabContainer, OS};
use gdnative::prelude::*;

use crate::utils::update_status;
use crate::{config, ConfigError, Error};

pub static mut STATUS: Option<Ref<LineEdit>> = None;
pub static mut CONFIG: Option<config::Config> = None;

#[derive(NativeClass)]
#[inherit(CanvasLayer)]
pub struct MainScreen {}

#[methods]
impl MainScreen {
    fn new(_: &CanvasLayer) -> Self {
        MainScreen {}
    }

    #[export]
    fn _ready(&mut self, o: TRef<CanvasLayer>) {
        let status = unsafe { o.get_node_as::<LineEdit>("VBoxContainer/Status").unwrap() };
        unsafe { STATUS = Some(status.claim()) };

        let os = OS::godot_singleton();
        os.center_window();

        let mut config = config::Config::new();

        match config.load(config::SAVE_DIR) {
            Ok(_) => {}
            Err(e) => match e {
                Error::ConfigError(e) => match e {
                    ConfigError::FileNotFound => {
                        update_status("No config file found");
                    }
                    _ => {
                        panic!("{:?}", e);
                    }
                },
                _ => {
                    panic!("Unexpected error {:?}", e);
                }
            },
        }

        unsafe { CONFIG = Some(config) };

        os.set_window_title(crate::app_name_and_version());

        let tab_container = unsafe {
            o.get_node_as::<TabContainer>("VBoxContainer/TabContainer")
                .unwrap()
        };

        update_status("Ready!");
    }
}
