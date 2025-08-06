//! Provides a clipboard abstraction to access the target system's clipboard.

#[cfg(all(feature = "copypasta", not(feature = "wasm")))]
use copypasta::{ClipboardContext, ClipboardProvider};


#[cfg(feature = "wasm")]
use web_sys::window;

use dioxus::{core::provide_root_context, prelude::*};

#[derive(Debug, PartialEq, Clone)]
pub enum ClipboardError {
    FailedToRead,
    FailedToSet,
    NotAvailable,
}


// Internal implementations - made pub(crate) to avoid private interface warnings
#[cfg(all(feature = "copypasta", not(feature = "wasm")))]
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct NativeClipboard {
    clipboard: Signal<Option<ClipboardContext>>,
}

#[cfg(all(feature = "wasm", not(feature = "async")))]
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct WasmClipboardSync {
    clipboard_cache: Signal<Option<String>>,
}

#[cfg(all(feature = "wasm", feature = "async"))]
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct WasmClipboardAsync;

/// Handle to access the ClipboardContext.
///
/// Use it through [use_clipboard].
#[derive(Clone, Copy, PartialEq)]
#[allow(private_interfaces)]
pub enum UseClipboard {
    #[cfg(all(feature = "copypasta", not(feature = "wasm")))]
    Native(NativeClipboard),
    
    #[cfg(all(feature = "wasm", not(feature = "async")))]
    WasmSync(WasmClipboardSync),
    
    #[cfg(all(feature = "wasm", feature = "async"))]
    WasmAsync(WasmClipboardAsync),
}

// Internal implementation methods
#[cfg(all(feature = "copypasta", not(feature = "wasm")))]
impl NativeClipboard {
    fn get(&mut self) -> Result<String, ClipboardError> {
        self.clipboard
            .write()
            .as_mut()
            .ok_or(ClipboardError::NotAvailable)?
            .get_contents()
            .map_err(|_| ClipboardError::FailedToRead)
    }

    fn set(&mut self, contents: String) -> Result<(), ClipboardError> {
        self.clipboard
            .write()
            .as_mut()
            .ok_or(ClipboardError::NotAvailable)?
            .set_contents(contents)
            .map_err(|_| ClipboardError::FailedToSet)
    }
}

#[cfg(all(feature = "wasm", not(feature = "async")))]
impl WasmClipboardSync {
    fn get(&mut self) -> Result<String, ClipboardError> {
        // Return cached value and trigger background refresh
        let cached_value = self.clipboard_cache.read().clone();

        // Spawn background refresh
        let mut cache_signal = self.clipboard_cache;
        let _ = spawn(async move {
            if let Ok(content) = read_clipboard_async().await {
                *cache_signal.write() = Some(content);
            }
        });

        cached_value.ok_or(ClipboardError::NotAvailable)
    }

    fn set(&mut self, contents: String) -> Result<(), ClipboardError> {
        // Update cache immediately
        *self.clipboard_cache.write() = Some(contents.clone());

        // Fire and forget async write
        let _ = spawn(async move {
            let _ = write_clipboard_async(&contents).await;
        });

        Ok(())
    }
}

#[cfg(all(feature = "wasm", feature = "async"))]
impl WasmClipboardAsync {
    async fn get(&mut self) -> Result<String, ClipboardError> {
        read_clipboard_async().await
    }

    async fn set(&mut self, contents: String) -> Result<(), ClipboardError> {
        write_clipboard_async(&contents).await
    }
}

// Public API - Sync implementations
#[cfg(not(feature = "async"))]
impl UseClipboard {
    pub fn get(&mut self) -> Result<String, ClipboardError> {
        match self {
            #[cfg(all(feature = "copypasta", not(feature = "wasm")))]
            UseClipboard::Native(clipboard) => clipboard.get(),
            
            #[cfg(all(feature = "wasm", not(feature = "async")))]
            UseClipboard::WasmSync(clipboard) => clipboard.get(),
        }
    }

    pub fn set(&mut self, contents: String) -> Result<(), ClipboardError> {
        match self {
            #[cfg(all(feature = "copypasta", not(feature = "wasm")))]
            UseClipboard::Native(clipboard) => clipboard.set(contents),
            
            #[cfg(all(feature = "wasm", not(feature = "async")))]
            UseClipboard::WasmSync(clipboard) => clipboard.set(contents),
        }
    }
}

// Public API - Async implementations  
#[cfg(feature = "async")]
impl UseClipboard {
    pub async fn get(&mut self) -> Result<String, ClipboardError> {
        match self {
            #[cfg(all(feature = "copypasta", not(feature = "wasm")))]
            UseClipboard::Native(clipboard) => {
                // Native async implementation - use sync clipboard directly since copypasta is already fast
                clipboard.clipboard
                    .write()
                    .as_mut()
                    .ok_or(ClipboardError::NotAvailable)?
                    .get_contents()
                    .map_err(|_| ClipboardError::FailedToRead)
            }
            
            #[cfg(all(feature = "wasm", feature = "async"))]
            UseClipboard::WasmAsync(clipboard) => clipboard.get().await,
        }
    }

    pub async fn set(&mut self, contents: String) -> Result<(), ClipboardError> {
        match self {
            #[cfg(all(feature = "copypasta", not(feature = "wasm")))]
            UseClipboard::Native(clipboard) => {
                clipboard.clipboard
                    .write()
                    .as_mut()
                    .ok_or(ClipboardError::NotAvailable)?
                    .set_contents(contents)
                    .map_err(|_| ClipboardError::FailedToSet)
            }
            
            #[cfg(all(feature = "wasm", feature = "async"))]
            UseClipboard::WasmAsync(clipboard) => clipboard.set(contents).await,
        }
    }
}

// WASM clipboard helper functions
#[cfg(feature = "wasm")]
async fn read_clipboard_async() -> Result<String, ClipboardError> {
    let window = window().ok_or(ClipboardError::NotAvailable)?;
    let navigator = window.navigator();
    let clipboard = navigator.clipboard();

    let promise = clipboard.read_text();
    let js_value = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|_| ClipboardError::FailedToRead)?;

    js_value.as_string().ok_or(ClipboardError::FailedToRead)
}

#[cfg(feature = "wasm")]
async fn write_clipboard_async(contents: &str) -> Result<(), ClipboardError> {
    let window = window().ok_or(ClipboardError::NotAvailable)?;
    let navigator = window.navigator();
    let clipboard = navigator.clipboard();

    let promise = clipboard.write_text(contents);
    wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|_| ClipboardError::FailedToSet)?;

    Ok(())
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
/// if let Ok(content) = clipboard.get() {
///     println!("{}", content);
/// }
///
/// // Write to the clipboard
/// clipboard.set("Hello, Dioxus!".to_string());;
///
/// ```
pub fn use_clipboard() -> UseClipboard {
    match try_consume_context() {
        Some(clipboard) => clipboard,
        None => {
            // Native implementation (default and async native)
            #[cfg(all(feature = "copypasta", not(feature = "wasm")))]
            {
                let clipboard_signal = Signal::new_in_scope(ClipboardContext::new().ok(), ScopeId::ROOT);
                let clipboard = UseClipboard::Native(NativeClipboard { clipboard: clipboard_signal });
                provide_root_context(clipboard)
            }

            // WASM hybrid sync implementation
            #[cfg(all(feature = "wasm", not(feature = "async")))]
            {
                let cache_signal = Signal::new_in_scope(None, ScopeId::ROOT);
                let clipboard = UseClipboard::WasmSync(WasmClipboardSync { clipboard_cache: cache_signal });
                provide_root_context(clipboard)
            }

            // WASM async implementation (stateless)
            #[cfg(all(feature = "wasm", feature = "async"))]
            {
                let clipboard = UseClipboard::WasmAsync(WasmClipboardAsync);
                provide_root_context(clipboard)
            }
        }
    }
}