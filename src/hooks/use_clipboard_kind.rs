use dioxus_lib::prelude::*;

use super::{use_clipboard_wasm::UseClipboardWasm, ClipboardError, UseClipboardDesktop};

#[derive(Clone, Copy)]
pub enum UseClipboard {
    Desktop(UseClipboardDesktop),
    Wasm(UseClipboardWasm),
}

impl UseClipboard {
    pub fn new() -> Self {
        client! {
        #[cfg(target_arch = "wasm32")]
        return Self::Wasm(UseClipboardWasm::new());
        }

        #[cfg(not(target_arch = "wasm32"))]
        return Self::Desktop(UseClipboardDesktop::new());
    }
    pub async fn set(&mut self, value: impl Into<String>) -> Result<(), ClipboardError> {
        match self {
            UseClipboard::Desktop(clipboard) => clipboard.set(value.into()),
            UseClipboard::Wasm(clipboard) => clipboard.set(value.into()).await,

        }
    }
    pub async fn get(&mut self) -> Result<String, ClipboardError> {
        match self {
            UseClipboard::Desktop(clipboard) => clipboard.get(),
            UseClipboard::Wasm(clipboard) => clipboard.get().await,
        }
    }
}
