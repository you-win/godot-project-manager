use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct AddonsScreen {}

#[methods]
impl AddonsScreen {
    fn new(_: &Control) -> Self {
        Self {}
    }
}
