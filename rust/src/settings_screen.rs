use gdnative::api::{HSplitContainer, Tree};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct SettingsScreen {}

#[methods]
impl SettingsScreen {
    fn new(_: &Control) -> Self {
        Self {}
    }

    #[export]
    fn _ready(&self, o: TRef<Control>) {
        let hsplit = unsafe { o.get_node_as::<HSplitContainer>("HSplitContainer").unwrap() };
        hsplit.set_split_offset((hsplit.get_rect().size.x * 0.3) as i64);

        let tree = unsafe { o.get_node_as::<Tree>("HSplitContainer/Tree").unwrap() };

        let root = tree.create_item(Null::null(), -1).unwrap();

        // let mut page_list = vec![];
        for child in hsplit.get_children().iter() {
            let node = match child.try_to_object::<Node>() {
                Ok(n) => n,
                Err(e) => panic!("Failed to cast Variant {}", e.to_string()),
            };

            // page_list.push(node);

            let item = tree.create_item(root, -1).unwrap();
        }
    }
}
