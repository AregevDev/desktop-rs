pub fn ret_environment() -> &'static str{
    if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "macos") {
        "MacOS"
    } else {
        env!("XDG_CURRENT_DESKTOP")
    }
}