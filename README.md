# desktop-rs
A simple and lightweight library that can detect the target platform and it's architecture and desktop environment.  
Since 1.0.0 this crate uses the 2018 edition

[![Build Status](https://travis-ci.com/AregevDev/desktop-rs.svg?branch=master)](https://travis-ci.com/AregevDev/desktop-rs)

### Brief example
Rust 2018
```rust
use desktop::Desktop;

fn main() {
    let d = Desktop::get();
    println!("We are running {} architecure {} using the {} desktop", d.os_name(), d.arch(), d.environment())
}
```

For the 2015 edition just add `extern crate desktop;` at the top of the file

[Documentation]()

### Issues
Feel free to hop over https://github.com/AregevDev/desktop-rs/issues

### Licence
Desktop is licenced under the terms of Apache-2.0 license