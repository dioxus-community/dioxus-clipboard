mod use_clipboard;
pub use use_clipboard::*;


#[cfg(feature = "wasm")]
mod clipboard_wasm;

#[cfg(feature = "wasm")]
pub use clipboard_wasm::*;
