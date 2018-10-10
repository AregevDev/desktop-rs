#[cfg(target_arch = "x86")]
fn arch() -> String {
    "i686".to_string()
}

#[cfg(target_arch = "x86_64")]
fn arch() -> String {
    "amd64".to_string()
}

#[cfg(target_arch = "arm")]
fn arch() -> String {
    "arm".to_string()
}

#[cfg(target_arch = "mips")]
fn arch() -> String {
    "mips".to_string()
}

pub fn ret_arch() -> String {
    arch()
}