use gdnative::api::{LineEdit, TabContainer, OS};
use gdnative::prelude::*;

use crate::{utils::update_status, ENGINE, STATUS};

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

        let mut engine = ENGINE.lock().unwrap();
        match engine.load_config() {
            Ok(_) => {}
            Err(e) => {
                update_status(format!(
                    "{:?}\nFailed to load config, using default config file",
                    e
                ));
            }
        }

        os.set_window_title(crate::app_name_and_version());

        let tab_container = unsafe {
            o.get_node_as::<TabContainer>("VBoxContainer/TabContainer")
                .unwrap()
        };

        update_status("Ready!");
    }

    #[export]
    fn _exit_tree(&self, _: &CanvasLayer) {
        match ENGINE.lock().unwrap().write_config() {
            Ok(_) => {}
            Err(e) => godot_error!("{}", e.to_string()),
        }
    }
}
