#[cfg(target_os = "linux")]
fn os_name() -> String {
    "Linux".to_string()
}

#[cfg(target_os = "macos")]
fn os_name() -> String {
    "MacOS".to_string()
}

#[cfg(target_os = "windows")]
fn os_name() ->  String {
    "Windows".to_string()
}

#[cfg(target_os = "ios")]
fn os_name() -> String{
    "IOS".to_string()
}

#[cfg(target_os = "android")]
fn os_name() -> String {
    "Android".to_string()
}

#[cfg(target_os = "freebsd")]
fn os_name() -> String {
    "FreeBSD".to_string()
}

pub fn ret_os_name() -> String {
    os_name()
}