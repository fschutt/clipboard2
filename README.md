# clipboard2

[![Build status](https://ci.appveyor.com/api/projects/status/i9m3ly8yfa3idhy0?svg=true)](https://ci.appveyor.com/project/fschutt/clipboard2)
[![Travis Build Status](https://travis-ci.org/fschutt/clipboard2.svg?branch=master)](https://travis-ci.org/fschutt/clipboard2)

Hard fork of https://github.com/aweinstock314/rust-clipboard

This library has better error handling than the original one,
the code is mostly copied.

# Usage

```rust
extern crate clipboard2;

use clipboard2::{Clipboard, SystemClipboard};

fn main() {
    let clipboard = SystemClipboard::new().unwrap();
    clipboard.set_string_contents(String::from("Hello")).unwrap();
    println!("{}", clipboard.get_string_contents().unwrap());
}
```