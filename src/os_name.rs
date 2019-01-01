#[cfg(target_os = "linux")]
const fn os_name() -> &'static str {
    "Linux"
}

#[cfg(target_os = "macos")]
const fn os_name() -> &'static str {
    "MacOS"
}

#[cfg(target_os = "windows")]
const fn os_name() ->  &'static str {
    "Windows"
}

#[cfg(target_os = "ios")]
const fn os_name() -> &'static str {
    "IOS"
}

#[cfg(target_os = "android")]
const fn os_name() -> &'static str {
    "Android"
}

#[cfg(target_os = "freebsd")]
const fn os_name() ->  &'static str {
    "FreeBSD"
}

#[cfg(target_os = "openbsd")]
const fn os_name() ->  &'static str {
    "OpenBSD"
}

#[cfg(target_os = "netbsd")]
const fn os_name() ->  &'static str {
    "NetBSD"
}

pub fn ret_os_name() ->  &'static str {
    os_name()
}