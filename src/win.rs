use clipboard_win::{get_clipboard_string, set_clipboard_string};

use clipboard_win::Clipboard as SystemClipboard;
use clipboard_win::formats;

pub struct WindowsClipboard {
	inner: SystemClipboard,
}

impl Clipboard for WindowsClipboard {
	type Output = Self;

	fn new() -> Result<Self::Output, ClipboardError> {
		Ok(WindowsClipboard {
			inner: SystemClipboard::new()?
		})
	}

	fn get_contents(&self) -> Result<String, ClipboardError> {
		Ok(get_clipboard_string()?)
	}

	fn set_contents(&self, contents: &String) -> Result<(), ClipboardError> {
		Ok(set_clipboard_string(&data)?)
	}
}