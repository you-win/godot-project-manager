use gdnative::{
    api::{HSplitContainer, VBoxContainer},
    prelude::*,
};

use crate::{nyi, utils::update_status};

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
    fn _ready(&self, o: TRef<Control>) {
        let hsplit = unsafe { o.get_node_as::<HSplitContainer>("HSplitContainer").unwrap() };
        hsplit.set_split_offset((hsplit.get_rect().size.x * 0.8) as i64);

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
        nyi!()
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
