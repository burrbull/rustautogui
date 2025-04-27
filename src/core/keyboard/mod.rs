use std::collections::HashMap;

use crate::errors::AutoGuiError;

cfg_select! {
    target_os = "windows" => {
        pub mod windows;
        pub use windows::Keyboard;
        type Key = u16;
    }
    target_os = "linux" => {
        pub mod linux;
        pub use linux::Keyboard;
        type Key = String;
    }
    target_os = "macos" => {
        pub mod macos;
        pub use macos::Keyboard;
        type Key = u16;
    }
}

fn get_keymap_key<'a>(target: &'a Keyboard, key: &str) -> Result<&'a (Key, bool), AutoGuiError> {
    let values = target
        .keymap
        .get(key)
        .ok_or(AutoGuiError::UnSupportedKey(format!(
            "{} key/command is not supported",
            key
        )))?;
    Ok(values)
}
