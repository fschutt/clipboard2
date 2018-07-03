use std::fmt::{self, Display, Formatter};

#[cfg(target_os="linux")]
use x11_clipboard::error::Error as X11Error;

use std::string::FromUtf8Error;
use std::error::Error;

#[derive(Debug)]
pub enum ClipboardError {
    Unimplemented,
    EncodingError(FromUtf8Error),
    #[cfg(target_os="linux")]
    X11ClipboardError(X11Error),
    #[cfg(target_os="macos")]
    MacOsClipboardError(MacOsError),
}

#[cfg(target_os="linux")]
impl From<X11Error> for ClipboardError {
    fn from(e: X11Error) -> Self {
        ClipboardError::X11ClipboardError(e)
    }
}

#[cfg(target_os="macos")]
impl From<MacOsError> for ClipboardError {
    fn from(e: MacOsError) -> Self {
        ClipboardError::MacOsClipboardError(e)
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg(target_os = "macos")]
pub enum MacOsError {
    PasteWriteObjectsError,
    ReadObjectsForClassesEmpty,
    ReadObjectsForClassesNull,
    PasteboardNotFound,
    NullPasteboard,
}

#[cfg(target_os = "macos")]
impl Error for MacOsError {
    fn description(&self) -> &str {
        use self::MacOsError::*;
        match *self {
            PasteWriteObjectsError => "Could not paste objects to clipboard",
            ReadObjectsForClassesEmpty => "Clipboard is empty",
            ReadObjectsForClassesNull => "No objects to read",
            PasteboardNotFound => "Pasteboard not found",
            NullPasteboard => "General pasteboard not found",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            PasteWriteObjectsError => "NSPasteboard @ writeObjects: returned false",
            ReadObjectsForClassesEmpty => "pasteboard @ readObjectsForClasses: returned empty",
            ReadObjectsForClassesNull => "pasteboard @ readObjectsForClasses: returned null",
            PasteboardNotFound => "NSPasteboard: class not found",
            NullPasteboard => "NSPasteboard @ generalPasteboard: returned null",
        }
    }
}

#[cfg(target_os = "macos")]
impl Display for MacOsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ClipboardError {
    fn description(&self) -> &str {
        use self::ClipboardError::*;
        match *self {
            Unimplemented => "Attempting to set the contents of the clipboard, \
                              which hasn't yet been implemented on this platform.",
            EncodingError(ref e) => e.description(),
            #[cfg(target_os="linux")]
            X11ClipboardError(ref e) => e.description(),
            #[cfg(target_os="macos")]
            MacOsClipboardError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        use self::ClipboardError::*;
        match *self {
            Unimplemented => None,
            EncodingError(ref e) => e.cause(),
            #[cfg(target_os="linux")]
            X11ClipboardError(ref e) => e.cause(),
            #[cfg(target_os="macos")]
            MacOsClipboardError(ref e) => e.cause(),
        }
    }
}

impl Display for ClipboardError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let cause_str = if let Some(cause) = self.cause() {
            format!("cause: {}", cause)
        } else {
            String::new()
        };
        write!(f, "Clipboard Error: {}\r\n{}", self.description(), cause_str)
    }
}

impl From<FromUtf8Error> for ClipboardError {
    fn from(e: FromUtf8Error) -> Self {
        ClipboardError::EncodingError(e)
    }
}