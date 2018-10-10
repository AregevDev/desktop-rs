use std::env;

pub fn ret_environment() -> String{
    if cfg!(target_os = "windows") {
        "Windows".to_string()
    } else if cfg!(target_os = "macos") {
        "MacOS".to_string()
    } else {
        let key = "XDG_CURRENT_DESKTOP";
        let env = match env::var(key) {
            Ok(v) => v,
            Err(_) => "".to_string(),
        };

        env
    }
}