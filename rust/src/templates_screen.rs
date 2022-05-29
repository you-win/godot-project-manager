use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct TemplatesScreen {}

#[methods]
impl TemplatesScreen {
    fn new(_: &Control) -> Self {
        Self {}
    }
}
