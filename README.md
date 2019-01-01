# desktop-rs
A simple and lightweight library that can detect the target platform and it's architecture and desktop environment.  
Since 1.0.0 this crate uses the 2018 edition

[![Build Status](https://travis-ci.com/AregevDev/desktop-rs.svg?branch=master)](https://travis-ci.com/AregevDev/desktop-rs)

### Brief example 
Rust 2015
```rust
extern crate desktop;

use desktop::Desktop;

fn main() {
    let d = Desktop::get();
    println!("We are running {} architecure {} using the {} desktop", d.os_name(), d.arch(), d.environment())
}
```