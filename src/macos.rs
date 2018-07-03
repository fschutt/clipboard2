use Clipboard;
use errors::{ClipboardError, MacOsError};

use objc::runtime::{Object, Class};
use objc_foundation::{
	INSArray, INSString, INSObject,
	NSArray, NSDictionary, NSString, NSObject
};
use objc_id::{Id, Owned};
use std::mem::transmute;

// required to bring NSPasteboard into the path of the class-resolver
#[link(name = "AppKit", kind = "framework")]
extern "C" { }

pub struct MacOsClipboard {
	pasteboard: Id<Object>,
}

/*
pub trait Clipboard {
	type Output;
	fn new() -> Result<Self::Output, ClipboardError>;
	fn get_contents(&self) -> Result<Vec<u8>, ClipboardError>;
	fn set_contents(&self, contents: Vec<u8>) -> Result<(), ClipboardError>;
}
*/

impl Clipboard for MacOsClipboard {

	type Output = Self;

	fn new() -> Result<Self::Output, ClipboardError> {
		let cls = Class::get("NSPasteboard").ok_or(MacOsError::PasteboardNotFound)?;
		let pasteboard: *mut Object = unsafe { msg_send![cls, generalPasteboard] };
		if pasteboard.is_null() {
		    return Err(MacOsClipboard::NullPasteboard.into());
		}
		let pasteboard: Id<Object> = unsafe { Id::from_ptr(pasteboard) };
		Ok(MacOsClipboard { pasteboard: pasteboard })
	}

	fn get_contents(&self) -> Result<Vec<u8>, ClipboardError> {
		let string_class: Id<NSObject> = {
		    let cls: Id<Class> = unsafe { Id::from_ptr(class("NSString")) };
		    unsafe { transmute(cls) }
		};
		let classes: Id<NSArray<NSObject, Owned>> = NSArray::from_vec(vec![string_class]);
		let options: Id<NSDictionary<NSObject, NSObject>> = NSDictionary::new();
		let string_array: Id<NSArray<NSString>> = unsafe {
		    let obj: *mut _ =
		        msg_send![self.pasteboard, readObjectsForClasses:&*classes options:&*options];
		    if obj.is_null() {
		        return Err(MacOsError::ReadObjectsForClassesNull.into());
		    }
		    Id::from_ptr(obj)
		};
		if string_array.count() == 0 {
		    Err(MacOsError::ReadObjectsForClassesEmpty.into())
		} else {
		    Ok(string_array[0].to_vec())
		}
	}

	fn get_string_contents(&self) -> Result<String, ClipboardError> {
		
	}

	fn set_contents(&self, contents: Vec<u8>) -> Result<(), ClipboardError> {
		let string_array = NSArray::from_slice(&contents);
		let _: usize = unsafe { msg_send![self.pasteboard, clearContents] };
		let success: bool = unsafe { msg_send![self.pasteboard, writeObjects:string_array] };
		if success {
		    Ok(())
		} else {
		    Err(MacOsError::PasteWriteObjectsError.into())
		}
	}
}

// This is a convenience function that both cocoa-rs and
// glutin define, which seems to depend on the fact that
// `Option::None` has the same representation as a null pointer
#[inline]
pub fn class(name: &str) -> *mut Class {
    unsafe { transmute(Class::get(name)) }
}