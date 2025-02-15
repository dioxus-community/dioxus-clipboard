//! Provides a clipboard abstraction to access the target system's clipboard.

use copypasta::{ClipboardContext, ClipboardProvider};
use dioxus_lib::prelude::*;

#[cfg(target_arch = "wasm32")]
use super::use_clipboard_wasm::UseClipboardWasm;
use super::use_clipboard_kind::UseClipboard;

#[derive(Debug, PartialEq, Clone)]
pub enum ClipboardError {
    FailedToRead,
    FailedToSet,
    NotAvailable,
}

/// Handle to access the ClipboardContext.
///
/// Use it through [use_clipboard].
#[derive(Clone, Copy, PartialEq)]
pub struct UseClipboardDesktop {
    pub(crate) clipboard: Signal<Option<ClipboardContext>>,
}

impl UseClipboardDesktop {
    pub(crate) fn new() -> Self {
        Self {
            clipboard: Signal::new_in_scope(ClipboardContext::new().ok(), ScopeId::ROOT),
        }
    }
    // Read from the clipboard
    pub fn get(&mut self) -> Result<String, ClipboardError> {
        self.clipboard
            .write()
            .as_mut()
            .ok_or(ClipboardError::NotAvailable)?
            .get_contents()
            .map_err(|_| ClipboardError::FailedToRead)
    }

    // Write to the clipboard
    pub fn set(&mut self, contents: String) -> Result<(), ClipboardError> {
        self.clipboard
            .write()
            .as_mut()
            .ok_or(ClipboardError::NotAvailable)?
            .set_contents(contents)
            .map_err(|_| ClipboardError::FailedToSet)
    }
}

/// Access the clipboard.
///
/// # Examples
///
/// ```rust,ignore
/// use dioxus_clipboard::prelude::use_clipboard;
///
/// // Get a handle to the clipboard
/// let mut clipboard = use_clipboard();
///
/// // Read the clipboard content
/// if let Ok(content) = clipboard.get().await {
///     println!("{}", content);
/// }
///
/// // Write to the clipboard
/// clipboard.set("Hello, Dioxus!".to_string()).await;;
///
/// ```
pub fn use_clipboard() -> UseClipboard {
    match try_consume_context() {
        Some(rt) => rt,
        None => provide_root_context(UseClipboard::new()),
    }
}
