use gdnative::api::{HSplitContainer, TextEdit, Tree};
use gdnative::prelude::*;

use crate::LOGS;

const STARTING_SCREEN: &str = "General";
const TREE_NAME: &str = "Tree";
const TREE_COL: i64 = 0;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct SettingsScreen {
    hsplit: Option<Ref<HSplitContainer>>,
    tree: Option<Ref<Tree>>,
}

#[methods]
impl SettingsScreen {
    fn new(_: &Control) -> Self {
        Self {
            hsplit: None,
            tree: None,
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
}
