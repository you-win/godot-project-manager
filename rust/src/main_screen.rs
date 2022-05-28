use gdnative::api::{
    Engine, GlobalConstants, InputEventMouseButton, InputEventMouseMotion, TabContainer, OS,
};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(CanvasLayer)]
pub struct MainScreen {
    title_bar_mouse_down: bool,
    title_bar_mouse_offset: Vector2,
}

#[methods]
impl MainScreen {
    fn new(_: &CanvasLayer) -> Self {
        MainScreen {
            title_bar_mouse_down: false,
            title_bar_mouse_offset: Vector2::ZERO,
        }
    }

    #[export]
    fn _ready(&self, o: TRef<CanvasLayer>) {
        let os = OS::godot_singleton();
        os.center_window();

        let tab_container =
            unsafe { o.get_node_as::<TabContainer>("VBoxContainer/PanelContainer/TabContainer") };

        let top_bar = unsafe { o.get_node_as::<Control>("VBoxContainer/Top").unwrap() };
        top_bar
            .connect(
                "gui_input",
                o,
                "_on_top_bar_input",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        let minimize_button = unsafe {
            o.get_node_as::<Button>("VBoxContainer/Top/Bar/Minimize")
                .unwrap()
        };
        minimize_button
            .connect("pressed", o, "_on_minimize", VariantArray::new_shared(), 0)
            .unwrap();

        let fullscreen_button = unsafe {
            o.get_node_as::<Button>("VBoxContainer/Top/Bar/Fullscreen")
                .unwrap()
        };
        fullscreen_button
            .connect(
                "pressed",
                o,
                "_on_fullscreen",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        let close_button = unsafe {
            o.get_node_as::<Button>("VBoxContainer/Top/Bar/Close")
                .unwrap()
        };
        close_button
            .connect("pressed", o, "_on_close", VariantArray::new_shared(), 0)
            .unwrap()
    }

    #[export]
    fn _on_top_bar_input(&mut self, _: &CanvasLayer, ie: Ref<InputEvent>) {
        let ie = unsafe { ie.assume_safe() };
        if !ie.is_class("InputEventMouse") {
            return;
        }

        if ie.is_class("InputEventMouseButton") {
            let ie = ie.cast::<InputEventMouseButton>().unwrap();
            if ie.button_index() != GlobalConstants::BUTTON_LEFT {
                return;
            }

            if self.title_bar_mouse_down != ie.is_pressed() {
                self.title_bar_mouse_offset = ie.global_position();
            }
            self.title_bar_mouse_down = ie.is_pressed();
        } else if self.title_bar_mouse_down {
            // Must be InputEventMouseMotion
            let ie = ie.cast::<InputEventMouseMotion>().unwrap();
            let os = OS::godot_singleton();
            let window_position = os.window_position();
            os.set_window_position(window_position.linear_interpolate(
                window_position + ie.global_position() - self.title_bar_mouse_offset,
                0.5,
            ));
        }
    }

    #[export]
    fn _on_minimize(&self, _: &CanvasLayer) {
        OS::godot_singleton().set_window_minimized(true);
    }

    #[export]
    fn _on_fullscreen(&self, _: &CanvasLayer) {
        OS::godot_singleton().set_window_fullscreen(!OS::godot_singleton().is_window_fullscreen());
    }

    #[export]
    fn _on_close(&self, o: TRef<CanvasLayer>) {
        unsafe { o.get_tree().unwrap().assume_safe().quit(-1) };
    }
}
