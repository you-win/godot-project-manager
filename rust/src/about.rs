use std::path::PathBuf;

use gdnative::api::control::LayoutPreset;
use gdnative::api::{Directory, File, ProjectSettings, RichTextLabel, WindowDialog, OS};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct About;

#[methods]
impl About {
    fn new(_: &Control) -> Self {
        Self
    }

    #[export]
    fn _ready(&self, o: TRef<Control>) {
        let contents = unsafe {
            o.get_node_as::<RichTextLabel>("ScrollContainer/VBoxContainer/RichTextLabel")
                .unwrap()
        };
        contents
            .connect(
                "meta_clicked",
                o,
                "_on_meta_clicked",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        let third_party_licenses = unsafe {
            o.get_node_as::<Button>("ScrollContainer/VBoxContainer/ThirdPartyLicenses")
                .unwrap()
        };
        third_party_licenses
            .connect(
                "pressed",
                o,
                "_on_third_party_licenses",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();
    }

    #[export]
    fn _on_meta_clicked(&self, _: &Control, data: String) {
        match OS::godot_singleton().shell_open(data) {
            Ok(_) => {}
            Err(e) => godot_print!("{:?}", e),
        }
    }

    #[export]
    fn _on_third_party_licenses(&self, o: TRef<Control>) {
        let dir = Directory::new();

        let path = PathBuf::from(
            ProjectSettings::godot_singleton()
                .globalize_path("res://resources/licenses")
                .to_string(),
        );

        match dir.open(&path.to_str().unwrap()) {
            Ok(_) => {}
            Err(e) => {
                godot_print!("Unable to open licences directory: {:?}", e);
                return;
            }
        }

        match dir.list_dir_begin(true, true) {
            Ok(_) => {}
            Err(e) => {
                godot_print!("Unable to iterate over licenses directory: {:?}", e);
                return;
            }
        }

        let mut text = String::new();

        let mut file_name = dir.get_next();
        while !file_name.is_empty() {
            let file = File::new();

            let mut path = path.clone();
            path.push(file_name.to_string());

            match file.open(path.to_str().unwrap(), File::READ) {
                Ok(_) => {}
                Err(e) => {
                    godot_print!("Unable to open license file: {:?}", e);
                    dir.list_dir_end();
                    return;
                }
            }

            text.push_str(file.get_as_text().to_string().as_str());
            text.push_str("\n");

            file_name = dir.get_next();
        }

        dir.list_dir_end();

        let popup = WindowDialog::new();
        let label = RichTextLabel::new();
        label.set_anchors_preset(LayoutPreset::WIDE.0, false);
        label.set_begin(Vector2::new(10.0, 10.0));
        label.set_end(Vector2::new(-10.0, -10.0));
        label.set_selection_enabled(true);
        label.set_text(&text);
        popup.add_child(label, false);

        let title = "Licenses";
        popup.set_name(&title);
        popup.set_title(&title);

        let popup = unsafe { popup.assume_shared() };

        let args = VariantArray::new();
        args.push(popup);

        let popup = unsafe { popup.assume_safe() };

        match popup.connect("popup_hide", o, "_delete", args.into_shared(), 0) {
            Ok(_) => {}
            Err(e) => {
                godot_print!("Unable to connect: {:?}", e);
                popup.queue_free();
                return;
            }
        }

        o.add_child(popup, false);
        popup.popup_centered_ratio(0.75);
    }

    #[export]
    fn _delete(&self, _: &Control, node: Ref<Node>) {
        unsafe {
            match node.assume_safe_if_sane() {
                Some(n) => n.queue_free(),
                None => {
                    godot_print!("Invalid node in _delete callback. Probable memory leak.");
                    return;
                }
            }
        };
    }
}
