#[cfg(target_os = "linux")]
fn os_name() -> &'static str {
    "Linux"
}

#[cfg(target_os = "macos")]
fn os_name() -> &'static str {
    "MacOS"
}

#[cfg(target_os = "windows")]
fn os_name() ->  &'static str {
    "Windows"
}

#[cfg(target_os = "ios")]
fn os_name() -> &'static str {
    "IOS"
}

#[cfg(target_os = "android")]
fn os_name() -> &'static str {
    "Android"
}

#[cfg(target_os = "freebsd")]
fn os_name() ->  &'static str {
    "FreeBSD"
}

#[cfg(target_os = "openbsd")]
fn os_name() ->  &'static str {
    "OpenBSD"
}

#[cfg(target_os = "netbsd")]
fn os_name() ->  &'static str {
    "NetBSD"
}

pub fn ret_os_name() ->  &'static str {
    os_name()
}