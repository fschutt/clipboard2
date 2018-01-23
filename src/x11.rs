use Clipboard;
use errors::ClipboardError;
use x11_clipboard::Clipboard as SystemClipboard;
use std::time::Duration;

pub struct X11Clipboard {
	inner: SystemClipboard,
}

impl Clipboard for X11Clipboard {
	type Output = Self;

	fn new() -> Result<Self::Output, ClipboardError> {
		Ok(X11Clipboard {
			inner: SystemClipboard::new()?
		})
	}

	fn get_contents(&self) -> Result<Vec<u8>, ClipboardError> {
		Ok(self.inner.load(
            self.inner.getter.atoms.clipboard, // TODO: .primary
            self.inner.getter.atoms.utf8_string,
            self.inner.getter.atoms.property,
            Duration::from_secs(3),
        )?)
	}

	fn set_contents(&self, contents: Vec<u8>) -> Result<(), ClipboardError> {
		Ok(self.inner.store(
		    self.inner.setter.atoms.clipboard, // TODO: .primary ?
		    self.inner.setter.atoms.utf8_string,
		    contents,
		)?)
	}
}