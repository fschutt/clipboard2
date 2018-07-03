#![allow(dead_code)]

pub enum ClipboardContentType {
	#[cfg(target_os="linux")]
	X11ContentType(X11ContentType),
	#[cfg(target_os="windows")]
	WinContentType(WinContentType),
	#[cfg(target_os="macos")]
	MacContent,
}

/// See:
#[cfg(target_os="linux")]
pub enum X11ContentType {
	AdobePortableDocumentFormat,
	ApplePict,
	/// A list of pixel values
	Background,
	/// A list of bitmap IDs
	Bitmap,
	/// The start and end of the selection in bytes
	CharacterPosition,
	Class,
	/// Any top-level window owned by the selection owner
	ClientWindow,
	/// A list of colormap IDs
	Colormap,
	/// The start and end column numbers
	ColumnNumber,
	/// Compound Text
	CompoundText,
	Delete,
	/// A list of drawable IDs
	Drawable,
	Eps,
	EpsInterchange,
	/// The full path name of a file
	FileName,
	/// A list of pixel values
	Foreground,
	HostName,
	InsertProperty,
	InsertSelection,
	/// The number of bytes in the selection
	Length,
	/// The start and end line numbers
	LineNumber,
	/// The number of disjoint parts of the selection
	ListLength,
	/// The name of the selected procedure
	Module,
	Multiple,
	Name,
	/// ISO Office Document Interchange Format
	Odif,
	/// The operating system of the owner client
	OwnerOs,
	/// A list of pixmap IDs
	Pixmap,
	Postscript,
	/// The name of the selected procedure
	Procedure,
	/// The process ID of the owner
	Process,
	/// ISO Latin-1 (+TAB+NEWLINE) text
	String,
	/// A list of valid target atoms
	Targets,
	/// The task ID of the owner
	Task,
	/// The text in the owner's choice of encoding
	Text,
	/// The timestamp used to acquire the selection
	Timestamp,
	/// The name of the user running the owner
	User,
}

/// See https://msdn.microsoft.com/en-us/library/windows/desktop/ff729168%28v=vs.85%29.aspx
#[cfg(target_os="windows")]
pub enum WinContentType {
	///  A handle to a bitmap (HBITMAP)
	Bitmap,
	/// Custom content type, used as backup if none of the formats are known
	Custom(u32),
	/// A memory object containing a BITMAPINFO structure followed by the bitmap bits.
	Dib,
	/// A memory object containing a BITMAPV5HEADER structure followed by
	/// the bitmap color space information and the bitmap bits.
	Dib5,
	/// Software Arts' Data Interchange Format.
	Dif,
	/// Bitmap display format associated with a private format. The hMem parameter must be a
	/// handle to data that can be displayed in bitmap format in lieu of the privately
	/// formatted data.
	DspBitmap,
	/// CF_DSPENHMETAFILE: Enhanced metafile display format associated with a private
	/// format. The hMem parameter must be a handle to data that can be displayed in
	/// enhanced metafile format in lieu of the privately formatted data.
	DspEnhancedMetaFile,
	/// CF_DSPMETAFILEPICT: Metafile-picture display format associated with a private
	/// format. The hMem parameter must be a handle to data that can be displayed in
	/// metafile-picture format in lieu of the privately formatted data.
	DspMetaFilePict,
	/// Text display format associated with a private format. The hMem parameter must
	/// be a handle to data that can be displayed in text format in lieu of the
	/// privately formatted data.
	DspText,
	/// A handle to an enhanced metafile (HENHMETAFILE).
	EnhancedMetaFile,
	/// Start of a range of integer values for application-defined
	/// GDI object clipboard formats.
	GdiObjectFirst,
	/// End of a range of integer values for application-defined GDI
	/// object clipboard formats.
	GdiObjectLast,
	/// A handle to type HDROP that identifies a list of files.
	HDrop,
	/// The data is a handle to the locale identifier associated
	/// with text in the clipboard.
	Locale,
	/// Handle to a metafile picture format as defined by the METAFILEPICT structure.
	MetaFilePict,
	/// Text format containing characters in the OEM character set.
	OemText,
	/// Owner-display format
	OwnerDisplay,
	/// Handle to a color palette
	Palette,
	/// Data for the pen extensions to the Microsoft Windows for Pen Computing
	PenData,
	/// Start of a range of integer values for private clipboard formats
	PrivateFirst,
	/// End of a range of integer values for private clipboard formats
	PrivateLast,
	/// Represents audio data more complex than can be represented in a CF_WAVE standard wave format
	Riff,
	/// Microsoft Symbolic Link (SYLK) format
	Sylk,
	/// ANSI text format
	Text,
	/// Tagged-image file format
	Tiff,
	/// UTF16 text format
	UnicodeText,
	/// Represents audio data in one of the standard wave formats
	Wave,
}

const CF_BITMAP: u32 = 2;
const CF_DIB: u32 = 8;
const CF_DIBV5: u32 = 17;
const CF_DIF: u32 = 5;
const CF_DSPBITMAP: u32 = 0x0082;
const CF_DSPENHMETAFILE: u32 = 0x008E;
const CF_DSPMETAFILEPICT: u32 = 0x0083;
const CF_DSPTEXT: u32 = 0x0081;
const CF_ENHMETAFILE: u32 = 14;
const CF_GDIOBJFIRST: u32 = 0x0300;
const CF_GDIOBJLAST: u32 = 0x03FF;
const CF_HDROP: u32 = 15;
const CF_LOCALE: u32 = 16;
const CF_METAFILEPICT: u32 = 3;
const CF_OEMTEXT: u32 = 7;
const CF_OWNERDISPLAY: u32 = 0x0080;
const CF_PALETTE: u32 = 9;
const CF_PENDATA: u32 = 10;
const CF_PRIVATEFIRST: u32 = 0x0200;
const CF_PRIVATELAST: u32 = 0x02FF;
const CF_RIFF: u32 = 11;
const CF_SYLK: u32 = 4;
const CF_TEXT: u32 = 1;
const CF_TIFF: u32 = 6;
const CF_UNICODETEXT: u32 = 13;
const CF_WAVE: u32 = 12;

#[cfg(target_os="windows")]
impl Into<u32> for WinContentType {
	fn into(self) -> u32 {
		match self {
			Bitmap => CF_BITMAP,
			Custom(a) => a,
			Dib => CF_DIB,
			Dib5 => CF_DIB5,
			Dif => CF_DIF,
			DspBitmap => CF_DSPBITMAP,
			DspEnhancedMetaFile => CF_DSPENHMETAFILE,
			DspMetaFilePict => CF_DSPMETAFILEPICT,
			DspText => CF_DSPTEXT,
			EnhancedMetaFile => CF_ENHMETAFILE,
			GdiObjectFirst => CF_GDIOBJFIRST,
			GdiObjectLast =>CF_GDIOBJLAST,
			HDrop => CF_HDROP,
			Locale => CF_LOCALE,
			MetaFilePict => CF_METAFILEPICT,
			OemText => CF_OEMTEXT,
			OwnerDisplay => CF_OWNERDISPLAY,
			Palette => CF_PALETTE,
			PenData => CF_PENDATA,
			PrivateFirst => CF_PRIVATEFIRST,
			PrivateLast => CF_PRIVATELAST,
			Riff => CF_RIFF,
			Sylk => CF_SYLK,
			Text => CF_TEXT,
			Tiff => CF_TIFF,
			UnicodeText => CF_UNICODETEXT,
			Wave => CF_WAVE,
		}
	}
}