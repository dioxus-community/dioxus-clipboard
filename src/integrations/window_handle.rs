use copypasta::{ClipboardContext, ClipboardProvider};
use dioxus_lib::{
    prelude::{provide_root_context, try_consume_context, ScopeId},
    signals::Signal,
};
use raw_window_handle::RawDisplayHandle;

use crate::hooks::UseClipboard;

/// Create a clipboard from a raw display handle, useful for Wayland.
///
/// # Safety
///
/// Since the type of the display is a raw pointer, it's the responsibility of the callee to make sure that the passed pointer is a valid Wayland display.
pub unsafe fn create_native_clipboard(
    display: RawDisplayHandle,
) -> Option<Box<dyn ClipboardProvider>> {
    #[allow(clippy::match_single_binding)]
    match display {
        #[cfg(target_os = "linux")]
        RawDisplayHandle::Wayland(d) => {
            let (_primary, clipboard) =
                copypasta::wayland_clipboard::create_clipboards_from_external(d.display.as_ptr());
            let clipboard: Box<dyn ClipboardProvider> = Box::new(clipboard);
            Some(clipboard)
        }
        _ => ClipboardContext::new().ok().map(|c| {
            let clipboard: Box<dyn ClipboardProvider> = Box::new(c);
            clipboard
        }),
    }
}

// Register the clipboard in the Root Scope
pub fn provide_native_clipboard(provider: Box<dyn ClipboardProvider>) {
    match try_consume_context::<UseClipboard>() {
        Some(mut rt) => rt.replace_with(provider),
        None => {
            let clipboard_signal = Signal::new_in_scope(provider, ScopeId::ROOT);
            provide_root_context(clipboard_signal);
        }
    };
}
