//! Provides a clipboard abstraction to access the target system's clipboard.

use web_sys::Clipboard;

use super::ClipboardError;

/// Handle to access the ClipboardContext.
///
/// Use it through [use_clipboard].
#[derive(Clone, PartialEq)]
pub struct ClipboardWasm {
    clipboard: Option<Clipboard>,
}

impl ClipboardWasm {
    // Read from the clipboard
    pub async fn get(&mut self) -> Result<String, ClipboardError> {
        wasm_bindgen_futures::JsFuture::from(
            self.clipboard
                .as_mut()
                .ok_or(ClipboardError::FailedToRead)?
                .read_text(),
        )
        .await
        .as_mut()
        .map_err(|_| ClipboardError::FailedToRead)?
        .as_string()
        .ok_or(ClipboardError::FailedToRead)
    }

    // Write to the clipboard
    pub async fn set(&mut self, contents: String) -> Result<(), ClipboardError> {
        wasm_bindgen_futures::JsFuture::from(
            self.clipboard
                .as_mut()
                .ok_or(ClipboardError::FailedToSet)?
                .write_text(&contents),
        )
        .await
        .as_mut()
        .map_err(|_| ClipboardError::FailedToSet)?;

        Ok(())
    }
    /// Access the clipboard.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use dioxus_clipboard::prelude::use_clipboard_wasm;
    /// // Becouse wasm only runs on the client we need to use a use_effect.
    /// // To get a handle to the clipboard
    /// let mut clipboard = use_signal(|| None);
    /// use_effect(move || clipboard.set(ClipboardWasm::new()));

    /// // Get the clipboard. This is None on the server
    /// if let Some(cb) = clipboard() {
    ///     // Get the content of the clipboard
    ///     if let Ok(content) = cb.get().await {
    ///         println!("{}", content);
    ///     }
    ///     // Write to the clipboard
    ///     cb.set("Hello World").await
    /// }   
    /// ```
    pub fn new() -> Option<Self> {
        Some(ClipboardWasm {
            clipboard: ::web_sys::window().map(|w| w.navigator().clipboard()),
        })
    }
    /// Access the clipboard.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use dioxus_clipboard::prelude::use_clipboard_wasm;
    /// // This can only be used on the client..
    /// // To get a handle to the clipboard
    /// let mut cb = ClipboardWasm::new_client_only();;
    /// 
    /// // Write to the clipboard
    /// cb.set("Hello World").await
    ///
    /// // Get the content of the clipboard
    /// if let Ok(content) = cb.get().await {
    ///     println!("{content}");
    /// }
    /// ```
    pub fn new_client_only() -> Self {
        ClipboardWasm {
            clipboard: ::web_sys::window().map(|w| w.navigator().clipboard()),
        }
    }
}
