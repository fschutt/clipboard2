use Clipboard;
use errors::ClipboardError;

use clipboard_win::{get_clipboard_string, set_clipboard_string};

use clipboard_win::Clipboard as SystemClipboard;
use clipboard_win::formats;

pub struct WindowsClipboard { }

impl Clipboard for WindowsClipboard {
	type Output = Self;

	fn new() -> Result<Self::Output, ClipboardError> {
		Ok(WindowsClipboard { })
	}

	fn get_contents(&self) 
	-> Result<(Vec<u8>, ClipboardContentType), ClipboardError> 
	{
		let clipboard = SystemClipboard::new()?;
		let formats_avail = clipboard.enum_formats();
		Ok(SystemClipboard::new()?.get()?)
	}

	fn set_contents(&self, contents: Vec<u8>, format: ClipboardContentType) 
	-> Result<(), ClipboardError>
	{

	}
/*
fn new() -> Result<Self::Output, ClipboardError>;
fn get_string_contents(&self) -> Result<String, ClipboardError>;
fn set_contents(&self, contents: Vec<u8>, format: ClipboardContentType) -> Result<(), ClipboardError>;
fn set_string_contents(&self, contents: String) -> Result<(), ClipboardError>;
*/
	fn set_contents(&self, contents: &Vec<u8>) -> Result<(), ClipboardError> {
		Ok(SystemClipboard::new()?.set(contents)?)
	}
}