use crate::os_name::ret_os_name;

pub fn ret_environment() -> &'static str {
    if cfg!(not(target_os = "linux")) {
        ret_os_name()
    } else {
        env!("XDG_CURRENT_DESKTOP")
    }
}