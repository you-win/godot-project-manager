use std::path::{Path, PathBuf};

use gdnative::api::control::SizeFlags;
use gdnative::api::{
    HBoxContainer, HSplitContainer, LineEdit, ProjectSettings, RichTextLabel, TextEdit, Tree,
    VBoxContainer,
};
use gdnative::prelude::*;

use crate::utils::update_status;
use crate::{ENGINE, LOGS};

const STARTING_SCREEN: &str = "General";
const TREE_NAME: &str = "Tree";
const TREE_COL: i64 = 0;

const HOME_ENV_VAR: &str = "$HOME";

#[derive(NativeClass)]
#[inherit(Control)]
pub struct SettingsScreen {
    hsplit: Option<Ref<HSplitContainer>>,
    tree: Option<Ref<Tree>>,

    scan_paths: Option<Ref<VBoxContainer>>,
    add_path: Option<Ref<LineEdit>>,
    add_path_button: Option<Ref<Button>>,
}

#[methods]
impl SettingsScreen {
    fn new(_: &Control) -> Self {
        Self {
            hsplit: None,
            tree: None,

            scan_paths: None,
            add_path: None,
            add_path_button: None,
        }
    }

    #[export]
    fn _ready(&mut self, o: TRef<Control>) {
        let hsplit = unsafe { o.get_node_as::<HSplitContainer>("HSplitContainer").unwrap() };
        hsplit.set_split_offset((hsplit.get_rect().size.x * 0.3) as i64);
        self.hsplit = Some(hsplit.claim());

        let tree = unsafe { o.get_node_as::<Tree>("HSplitContainer/Tree").unwrap() };
        self.tree = Some(tree.claim());

        let root = tree.create_item(Null::null(), -1).unwrap();
        let pages = Dictionary::new();

        for child in hsplit.get_children().iter() {
            let node = match child.try_to_object::<Node>() {
                Ok(n) => n,
                Err(e) => panic!("Failed to cast Variant {}", e.to_string()),
            };

            let name = unsafe { node.assume_safe() }
                .name()
                .capitalize()
                .to_string();
            if name == TREE_NAME {
                continue;
            }

            pages.insert(&name, node);

            let item = tree.create_item(root, -1).unwrap();
            let item = unsafe { item.assume_safe() };
            item.set_text(TREE_COL, &name);

            if name == STARTING_SCREEN {
                item.select(TREE_COL);
            }
        }

        // NOTE This needs to be connected after creating all tree items
        // otherwise we run into re-entrant call errors
        tree.connect(
            "item_selected",
            o,
            "_on_tree_item_selected",
            VariantArray::new_shared(),
            0,
        )
        .unwrap();

        let logs = unsafe {
            o.get_node_as::<TextEdit>("HSplitContainer/Logs/TextEdit")
                .unwrap()
        };
        unsafe { LOGS = Some(logs.claim()) };

        let scan_paths_list = unsafe {
            o.get_node_as::<VBoxContainer>(
                "HSplitContainer/General/VBoxContainer/ScanPaths/VBoxContainer",
            )
            .unwrap()
        };
        self.scan_paths = Some(scan_paths_list.claim());

        let add_path_button = unsafe {
            o.get_node_as::<Button>(
                "HSplitContainer/General/VBoxContainer/ScanPaths/VBoxContainer/AddPath/Button",
            )
            .unwrap()
        };
        self.add_path_button = Some(add_path_button.claim());
        add_path_button
            .connect("pressed", o, "_on_add_path", VariantArray::new_shared(), 0)
            .unwrap();

        let add_path_line_edit = unsafe {
            o.get_node_as::<LineEdit>(
                "HSplitContainer/General/VBoxContainer/ScanPaths/VBoxContainer/AddPath/LineEdit",
            )
            .unwrap()
        };
        add_path_line_edit
            .connect(
                "text_entered",
                o,
                "_on_add_path_text_enter_pressed",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();
        add_path_line_edit
            .connect(
                "text_changed",
                o,
                "_on_add_path_text_changed",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();
        self.add_path = Some(add_path_line_edit.claim());
    }

    #[export]
    fn _on_tree_item_selected(&self, o: TRef<Control>) {
        let tree = unsafe { self.tree.unwrap().assume_safe() };

        let item = unsafe { tree.get_selected().unwrap().assume_safe() };
        let name = item.get_text(TREE_COL);

        let hsplit = unsafe { self.hsplit.unwrap().assume_safe() };
        for child in hsplit.get_children().iter() {
            let control = unsafe { child.to_object::<Control>().unwrap().assume_safe() };
            if control.name().to_string() == TREE_NAME {
                continue;
            }

            control.set_visible(false);
        }

        let node = unsafe { hsplit.get_node(name).unwrap().assume_safe() };
        node.cast::<Control>().unwrap().set_visible(true);
    }

    #[export]
    fn _on_add_path_text_changed(&mut self, _: &Control, text: GodotString) {
        let mut text = ProjectSettings::godot_singleton()
            .globalize_path(text)
            .to_string();

        text = text.replace(
            HOME_ENV_VAR,
            home::home_dir()
                .unwrap_or_else(|| {
                    update_status("Unable to compute user home dir");
                    PathBuf::from("/")
                })
                .to_str()
                .unwrap(),
        );

        unsafe { self.add_path_button.unwrap().assume_safe() }.set_disabled(
            if Path::new(&text).exists() {
                false
            } else {
                true
            },
        );
    }

    fn _on_add_path_text_enter_pressed(&mut self, o: &Control, _: GodotString) {
        self._on_add_path(o)
    }

    #[export]
    fn _on_add_path(&mut self, _: &Control) {
        let line_edit = unsafe { self.add_path.unwrap().assume_safe() };
        let mut text = line_edit.text().to_string();
        text = text.replace(
            HOME_ENV_VAR,
            home::home_dir()
                .unwrap_or_else(|| {
                    update_status("Unable to compute user home dir");
                    PathBuf::from("/")
                })
                .to_str()
                .unwrap(),
        );

        line_edit.set_text("");

        {
            let mut engine = ENGINE.lock().unwrap();
            let config = engine.config_mut();

            config.scan_paths.push(text.clone());
        }

        self.add_scan_path_item(&text);
    }

    #[export]
    fn _delete_node(&self, _: &Control, node: Ref<Node>) {
        unsafe {
            match node.assume_safe_if_sane() {
                Some(n) => n.queue_free(),
                None => {
                    update_status("Tried to delete non-existent node");
                    return;
                }
            }
        }
    }

    #[export]
    pub fn update_from_config(&mut self, _: &Control) {
        let mut engine = ENGINE.lock().unwrap();

        let config = engine.config_mut();

        for path in config.scan_paths.iter() {
            self.add_scan_path_item(path);
        }
    }

    fn add_scan_path_item(&mut self, text: &String) {
        let paths = unsafe { self.scan_paths.unwrap().assume_safe() };

        let item: Instance<ScanPathItem, Unique> = ScanPathItem::new_instance();
        item.map_mut(|item, instance| {
            item.apply_text(instance, text);
        })
        .unwrap();

        paths.add_child(item, false);
    }
}

#[derive(NativeClass)]
#[inherit(HBoxContainer)]
pub struct ScanPathItem {
    text: String,
}

#[methods]
impl ScanPathItem {
    fn new(o: TRef<HBoxContainer>) -> Self {
        let label = LineEdit::new();
        label.set_h_size_flags(SizeFlags::EXPAND_FILL.0);
        label.set_editable(false);

        let button = Button::new();
        button.set_text("Remove");
        button
            .connect("pressed", o, "_delete", VariantArray::new_shared(), 0)
            .unwrap();

        o.add_child(label, false);
        o.add_child(button, false);

        Self {
            text: "".to_string(),
        }
    }

    fn apply_text(&mut self, o: TRef<HBoxContainer, Unique>, text: &String) {
        let label = unsafe {
            o.get_child(0)
                .unwrap()
                .assume_safe()
                .cast::<LineEdit>()
                .unwrap()
        };
        label.set_text(text);
        o.set_name(text);

        self.text = text.clone();
    }

    #[export]
    fn _delete(&self, o: TRef<HBoxContainer>) {
        let mut engine = ENGINE.lock().unwrap();
        let config = engine.config_mut();

        match config.scan_paths.binary_search(&self.text) {
            Ok(i) => {
                config.scan_paths.remove(i);
            }
            Err(_) => {
                update_status(format!("Scan path {} not found", &self.text));
            }
        };

        o.queue_free();
    }
}
