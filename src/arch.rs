#[cfg(target_arch = "x86")]
const fn arch() -> &'static str {
    "i686"
}

#[cfg(target_arch = "x86_64")]
const fn arch() -> &'static str {
    "amd64"
}

#[cfg(target_arch = "arm")]
const fn arch() -> &'static str {
    "arm"
}

#[cfg(target_arch = "mips")]
const fn arch() -> &'static str {
    "mips"
}

pub const fn ret_arch() -> &'static str {
    arch()
}