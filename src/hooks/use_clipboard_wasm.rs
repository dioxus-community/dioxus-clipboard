//! Provides a clipboard abstraction to access the target system's clipboard.

use dioxus_lib::prelude::*;

use super::ClipboardError;

/// Handle to access the ClipboardContext.
///
/// Use it through [use_clipboard].
#[derive(Clone, PartialEq, Copy)]
pub struct UseClipboardWasm {
    pub clipboard: Signal<Option<web_sys::Clipboard>>,
}

impl UseClipboardWasm {
    pub(crate) fn new() -> Self {
        Self {
            clipboard: Signal::new_in_scope(
                ::web_sys::window().map(|w| w.navigator().clipboard()),
                ScopeId::ROOT,
            ),
        }
    }
    // Read from the clipboard
    pub async fn get(&self) -> Result<String, ClipboardError> {
        wasm_bindgen_futures::JsFuture::from(
            self.clipboard
                .read()
                .as_ref()
                .ok_or(ClipboardError::NotAvailable)?
                .read_text(),
        )
        .await
        .as_mut()
        .map_err(|_| ClipboardError::FailedToRead)?
        .as_string()
        .ok_or(ClipboardError::FailedToRead)
    }
    // Write to the clipboard
    pub async fn set(&mut self, contents: impl Into<String>) -> Result<(), ClipboardError> {
        wasm_bindgen_futures::JsFuture::from(
            self.clipboard
                .write()
                .as_mut()
                .ok_or(ClipboardError::NotAvailable)?
                .write_text(&contents.into()),
        )
        .await
        .as_mut()
        .map_err(|_| ClipboardError::FailedToSet)?;

        Ok(())
    }
}
