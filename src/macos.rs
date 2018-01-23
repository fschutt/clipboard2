pub struct MacOsClipboard;

impl Clipboard for MacOsClipboard {
	type Output = Self;

	fn new() -> Result<Self::Output, ClipboardError> {
		Ok(MacOsClipboard)
	}

	fn get_contents(&self) -> Result<String, ClipboardError> {
		Ok(String::new())
	}

	fn set_contents(&self, contents: &String) -> Result<(), ClipboardError> {
		Ok(())
	}
}