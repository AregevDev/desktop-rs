#[cfg(target_vendor = "apple")]
fn vendor() -> &'static str {
    "Apple"
}

#[cfg(target_vendor = "pc")]
fn vendor() -> &'static str {
    "PC"
}

#[cfg(target_vendor = "unknown")]
fn vendor() -> &'static str {
    "Unknown"
}

pub fn ret_vendor() -> &'static str {
    vendor()
}