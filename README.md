# clipboard2

Improved clipboard handling (similar to rust-clipboard)

This is a library for better, simpler clipboard handling than the existing rust-clipboard crate.

# Usage 

```
extern crate clipboard2;

use clipboard2::{Clipboard, SystemClipboard};

fn main() {
	let clipboard = SystemClipboard::new().unwrap();
	clipboard.set_contents("Hello".as_bytes().to_vec()).unwrap();
	println!("{}", String::from_utf8(clipboard.get_contents().unwrap()).unwrap());
}
```

Currently only tested on X11, does not work on MacOS.
