mod about;
mod main_screen;
mod projects;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<main_screen::MainScreen>();

    handle.add_class::<projects::Projects>();
    handle.add_class::<about::About>();
}

godot_init!(init);
