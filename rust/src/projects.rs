use gdnative::{api::HSplitContainer, prelude::*};

#[derive(NativeClass)]
#[inherit(Control)]
pub struct Projects;

#[methods]
impl Projects {
    fn new(_: &Control) -> Self {
        Self
    }

    #[export]
    fn _ready(&self, o: TRef<Control>) {
        let hsplit = unsafe { o.get_node_as::<HSplitContainer>("HSplitContainer").unwrap() };
        hsplit.set_split_offset((hsplit.get_rect().size.x * 0.8) as i64);
    }
}
