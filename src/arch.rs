#[cfg(target_arch = "x86")]
fn arch() -> &'static str {
    "i686"
}

#[cfg(target_arch = "x86_64")]
fn arch() -> &'static str {
    "amd64"
}

#[cfg(target_arch = "arm")]
fn arch() -> &'static str {
    "arm"
}

#[cfg(target_arch = "mips")]
fn arch() -> &'static str {
    "mips"
}

pub fn ret_arch() -> &'static str {
    arch()
}