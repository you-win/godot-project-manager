use std::fmt::Display;

use gdnative::prelude::*;

use crate::{LOGS, STATUS};

pub trait Stringable: ToString + AsRef<str> + Display {}

impl Stringable for &str {}
impl Stringable for String {}

pub fn update_status<T: Stringable>(text: T) {
    let date = chrono::Local::now();
    let text = format!("[{}] {}", date.format("%H:%M:%S"), &text);

    // Always print using Godot's builtin logger
    godot_print!("{}", &text);
    unsafe {
        if STATUS.is_none() {
            godot_error!("Status element is empty");
            return;
        }
        if LOGS.is_none() {
            godot_error!("Logs element is empty");
            return;
        }

        match STATUS.unwrap().assume_safe_if_sane() {
            Some(s) => {
                s.set_text(&text);
                match LOGS.unwrap().assume_safe_if_sane() {
                    Some(l) => {
                        let old_text = l.text();
                        l.set_text(format!("{}{}\n", old_text, &text));
                    }
                    None => godot_error!("Logs element is no longer valid"),
                }
            }
            None => godot_error!("Status element is no longer valid"),
        }
    }

    unsafe {
        match STATUS {
            Some(s) => match s.assume_safe_if_sane() {
                Some(s) => s.set_text(text),
                None => godot_error!("Status element no longer valid"),
            },
            None => godot_error!("Status element not found"),
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
