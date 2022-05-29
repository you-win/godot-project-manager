use gdnative::prelude::*;

use crate::main_screen::STATUS;

pub trait Stringable: ToString + AsRef<str> {}

impl Stringable for &str {}
impl Stringable for String {}

pub fn update_status<T: Stringable>(text: T) {
    unsafe {
        match STATUS {
            Some(s) => match s.assume_safe_if_sane() {
                Some(s) => s.set_text(text),
                None => godot_print!("Status element no longer valid"),
            },
            None => godot_print!("Status element not found"),
        }
    }
}

#[macro_export]
macro_rules! nyi {
    () => {{
        // TODO ??? https://stackoverflow.com/questions/38088067/equivalent-of-func-or-function-in-rust
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        let name = match &name[..name.len() - 3].rfind(":") {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        };

        crate::utils::update_status(format!("{} not yet implemented", name))
    }};
}
