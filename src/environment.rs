use std::env;

pub fn ret_environment() -> &'static str{
    if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "macos") {
        "MacOS"
    } else {
        let key = "XDG_CURRENT_DESKTOP";
        match env::var(key) {
            Ok(_) => key,
            Err(_) => "Unknown",
        }
    }
}