//! Improved cross-platform clipboard library

#[cfg(target_os="windows")]
extern crate clipboard_win;
#[cfg(target_os="linux")]
extern crate x11_clipboard;
#[cfg(target_os="macos")]
#[macro_use]
extern crate objc;
#[cfg(target_os="macos")]
extern crate objc_id;
#[cfg(target_os="macos")]
extern crate objc_foundation;

mod errors;
mod clipboard_metadata;

pub use errors::ClipboardError;

pub trait Clipboard {
	type Output;
	fn new() -> Result<Self::Output, ClipboardError>;
	fn get_contents(&self) -> Result<Vec<u8>, ClipboardError>;
	fn set_contents(&self, contents: Vec<u8>) -> Result<(), ClipboardError>;
}

#[cfg(target_os="windows")]
pub mod win;
#[cfg(target_os="windows")]
pub use win::WindowsClipboard as SystemClipboard;

#[cfg(target_os="linux")]
pub mod x11;
#[cfg(target_os="linux")]
pub use x11::X11Clipboard as SystemClipboard;

#[cfg(target_os="macos")]
pub mod macos;
#[cfg(target_os="macos")]
pub use macos::MacOsClipboard as SystemClipboard;

#[cfg(test)]
mod tests;