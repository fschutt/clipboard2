extern crate clipboard2;

use clipboard2::{Clipboard, SystemClipboard};

fn main() {
	let clipboard = SystemClipboard::new().unwrap();
	clipboard.set_contents("Hello".as_bytes().to_vec()).unwrap();
	println!("{}", String::from_utf8(clipboard.get_contents().unwrap()).unwrap());
}