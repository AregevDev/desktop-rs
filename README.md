# desktop-rs
A simple and lightweight library that can detect the target platform and it's architecture and desktop environment.

[![Build Status](https://travis-ci.com/AregevDev/desktop-rs.svg?branch=master)](https://travis-ci.com/AregevDev/desktop-rs)

### Brief example 
```rust
extern crate desktop;

use desktop::Desktop;

fn main() {
    let d = Desktop::get();
    println!("We are running {} architecure {} using the {} desktop", d.os_name(), d.arch(), d.environment())
}
```
