use std::cell::RefCell;

use gdnative::{
    api::{
        BoxContainer, HBoxContainer, HSplitContainer, TextureButton, TextureRect, VBoxContainer,
    },
    prelude::*,
};

use crate::{config::Project, nyi, utils::update_status};

#[derive(NativeClass)]
#[inherit(Control)]
pub struct ProjectsScreen {
    project_list: Option<Ref<VBoxContainer>>,
    selected_project: Option<Ref<Control>>,
}

#[methods]
impl ProjectsScreen {
    fn new(_: &Control) -> Self {
        Self {
            project_list: None,
            selected_project: None,
        }
    }

    #[export]
    fn _ready(&mut self, o: TRef<Control>) {
        let hsplit = unsafe { o.get_node_as::<HSplitContainer>("HSplitContainer").unwrap() };
        hsplit.set_split_offset((hsplit.get_rect().size.x * 0.8) as i64);

        let project_list = unsafe {
            o.get_node_as::<VBoxContainer>("HSplitContainer/Left/Project/List")
                .unwrap()
        };
        self.project_list = Some(project_list.claim());

        let scan_button = unsafe {
            o.get_node_as::<Button>("HSplitContainer/Right/Option/List/Scan")
                .unwrap()
        };
        scan_button
            .connect("pressed", o, "_on_scan", VariantArray::new_shared(), 0)
            .unwrap();

        let new_project_button = unsafe {
            o.get_node_as::<Button>("HSplitContainer/Right/Option/List/NewProject")
                .unwrap()
        };
        new_project_button
            .connect(
                "pressed",
                o,
                "_on_new_project",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        let import_button = unsafe {
            o.get_node_as::<Button>("HSplitContainer/Right/Option/List/Import")
                .unwrap()
        };
        import_button
            .connect("pressed", o, "_on_import", VariantArray::new_shared(), 0)
            .unwrap();

        let open_button = unsafe {
            o.get_node_as::<Button>("HSplitContainer/Right/Option/List/Open")
                .unwrap()
        };
        open_button
            .connect("pressed", o, "_on_open", VariantArray::new_shared(), 0)
            .unwrap();

        let rename_button = unsafe {
            o.get_node_as::<Button>("HSplitContainer/Right/Option/List/Rename")
                .unwrap()
        };
        rename_button
            .connect("pressed", o, "_on_rename", VariantArray::new_shared(), 0)
            .unwrap();

        let remove_button = unsafe {
            o.get_node_as::<Button>("HSplitContainer/Right/Option/List/Remove")
                .unwrap()
        };
        remove_button
            .connect("pressed", o, "_on_remove", VariantArray::new_shared(), 0)
            .unwrap();

        let remove_missing_button = unsafe {
            o.get_node_as::<Button>("HSplitContainer/Right/Option/List/RemoveMissing")
                .unwrap()
        };
        remove_missing_button
            .connect(
                "pressed",
                o,
                "_on_remove_missing",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();
    }

    #[export]
    fn _on_scan(&self, _: &Control) {
        let engine = crate::ENGINE.lock().unwrap();

        let projects = engine.scan_for_projects();
        drop(engine);

        let list = unsafe { self.project_list.unwrap().assume_safe() };

        for project in projects.iter() {
            let item: Instance<ProjectItem, Unique> = ProjectItem::new_instance();
            // let item = unsafe { item.into_shared().assume_safe() };

            item.map(|item, _instance| {
                item.apply_data(project);
            })
            .unwrap();

            list.add_child(item, false);

            // let item = item.claim();
        }
    }

    #[export]
    fn _on_new_project(&self, _: &Control) {
        nyi!()
    }

    #[export]
    fn _on_import(&self, _: &Control) {
        nyi!()
    }

    #[export]
    fn _on_open(&self, _: &Control) {
        nyi!()
    }

    #[export]
    fn _on_rename(&self, _: &Control) {
        nyi!()
    }

    #[export]
    fn _on_remove(&self, _: &Control) {
        nyi!()
    }

    #[export]
    fn _on_remove_missing(&self, _: &Control) {
        nyi!()
    }
}

/// Based on https://github.com/godotengine/godot/blob/3.4/editor/project_manager.cpp#L923
#[derive(NativeClass)]
#[inherit(HBoxContainer)]
pub struct ProjectItem {
    hover: RefCell<bool>,

    hb: Ref<HBoxContainer>,
    favorite_button: Ref<TextureButton>,
    icon: Ref<TextureRect>,
}

#[methods]
impl ProjectItem {
    fn new(o: TRef<HBoxContainer>) -> Self {
        let favorite_icon = o.get_icon("Favorites", "EditorIcons").unwrap();

        let favorite = TextureButton::new();
        favorite.set_name("FavoriteButton");
        favorite.set_normal_texture(favorite_icon);
        favorite.set_mouse_filter(Control::MOUSE_FILTER_PASS);
        favorite
            .connect(
                "pressed",
                o,
                "_favorite_pressed",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        let icon = TextureRect::new();
        icon.set_texture(o.get_icon("ProjectIconLoading", "EditorIcons").unwrap());
        icon.set_v_size_flags(Control::SIZE_SHRINK_CENTER);

        Self {
            hover: RefCell::new(false),

            hb: o.claim(),
            favorite_button: favorite.into_shared(),
            icon: icon.into_shared(),
        }
    }

    fn apply_data(&self, project: &Project) {
        let hb = unsafe { self.hb.assume_safe() };
        hb.connect("draw", hb, "_panel_draw", VariantArray::new_shared(), 0)
            .unwrap();
        hb.connect(
            "gui_input",
            hb,
            "_panel_input",
            VariantArray::new_shared(),
            0,
        )
        .unwrap();

        // TODO add editor scaling here?
        hb.add_constant_override("separation", 10);

        let favorite_box = VBoxContainer::new();
        favorite_box.set_name("FavoriteBox");

        favorite_box.add_child(unsafe { self.favorite_button.assume_safe() }, false);
        favorite_box.set_alignment(BoxContainer::ALIGN_CENTER);

        hb.add_child(favorite_box, false);

        let icon = unsafe { self.icon.assume_safe() };
        if project.missing {
            icon.set_modulate(Color::from_rgba(1.0, 1.0, 1.0, 0.5));
        }

        hb.add_child(icon, false);

        let vb = unsafe { VBoxContainer::new().assume_shared().assume_safe() };
        vb.set_h_size_flags(Control::SIZE_EXPAND_FILL);
        if project.grayed {
            vb.set_modulate(Color::from_rgba(1.0, 1.0, 1.0, 0.5));
        }

        hb.add_child(vb, false);

        let ec = Control::new();
        ec.set_custom_minimum_size(Vector2::new(0.0, 1.0));
        ec.set_mouse_filter(Control::MOUSE_FILTER_PASS);

        vb.add_child(ec, false);

        let title = Label::new();
        title.set_name("Title");
        title.set_clip_text(true);
        title.set_text(if !project.missing {
            project.name.to_string()
        } else {
            hb.tr("Missing Project").to_string()
        });

        vb.add_child(title, false);

        let path_hb = unsafe { HBoxContainer::new().assume_shared().assume_safe() };
        path_hb.set_h_size_flags(Control::SIZE_EXPAND_FILL);

        vb.add_child(path_hb, false);

        let show = unsafe { Button::new().assume_shared().assume_safe() };
        show.set_button_icon(
            hb.get_icon(
                if !project.missing {
                    "Load"
                } else {
                    "FileBroken"
                },
                "EditorIcons",
            )
            .unwrap(),
        );
        if !project.grayed {
            show.set_modulate(Color::from_rgba(1.0, 1.0, 1.0, 0.5));
        }

        path_hb.add_child(show, false);

        if !project.missing {
            let binds = VariantArray::new();
            binds.push(project.path.to_string());
            show.connect("pressed", hb, "_show_project", binds.into_shared(), 0)
                .unwrap();
            show.set_tooltip(hb.tr("Show in File Manager"));
        } else {
            show.set_tooltip(hb.tr("Error: Project is missing on the filesystem."));
        }

        let fpath = unsafe { Label::new().assume_shared().assume_safe() };
        fpath.set_text(project.path.to_string());

        path_hb.add_child(fpath, false);

        fpath.set_h_size_flags(Control::SIZE_EXPAND_FILL);
        fpath.set_modulate(Color::from_rgba(1.0, 1.0, 1.0, 0.5));
        fpath.set_clip_text(true);
    }

    #[export]
    fn set_is_favorite(&self, _: &HBoxContainer, fav: bool) {
        unsafe { self.favorite_button.assume_safe() }.set_modulate(if fav {
            Color::from_rgba(1.0, 1.0, 1.0, 1.0)
        } else {
            Color::from_rgba(1.0, 1.0, 1.0, 0.2)
        });
    }

    #[export]
    fn _notification(&self, o: &HBoxContainer, p_what: i64) {
        match p_what {
            Control::NOTIFICATION_MOUSE_ENTER => *self.hover.borrow_mut() = true,
            Control::NOTIFICATION_MOUSE_EXIT => *self.hover.borrow_mut() = false,
            CanvasItem::NOTIFICATION_DRAW => {
                if *self.hover.borrow() {
                    let hb = unsafe { self.hb.assume_safe() };
                    hb.draw_style_box(
                        hb.get_stylebox("hover", "Tree").unwrap(),
                        Rect2 {
                            position: Vector2::ZERO,
                            size: hb.size() - Vector2::new(10.0, 0.0),
                        },
                    )
                }
            }
            _ => {}
        }
    }

    #[export]
    fn _favorite_pressed(&self, o: TRef<HBoxContainer>) {
        //
    }

    #[export]
    fn _panel_draw(&self, o: TRef<HBoxContainer>) {
        //
    }

    #[export]
    fn _panel_input(&self, o: TRef<HBoxContainer>, ie: Ref<InputEvent>) {
        //
    }

    #[export]
    fn _show_project(&self, o: TRef<HBoxContainer>, path: GodotString) {}
}
