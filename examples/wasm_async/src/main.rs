//! WASM Async Clipboard Example
//!
//! Tests the async clipboard API with wasm + async features
//!
//! Run with:
//!
//! ```sh
//! dx serve --web
//! ```

#![allow(non_snake_case, unused)]
use dioxus::logger::tracing;
use dioxus::prelude::*;
use dioxus_clipboard::prelude::*;
use serde::{Deserialize, Serialize};

fn app() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| "...".to_string());
    let mut clipboard = use_clipboard();
    let server_future = use_server_future(get_server_data)?;

    rsx! {
        document::Link { href: asset!("/assets/hello.css"), rel: "stylesheet" }
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
        button {
            onclick: move |_| async move {
                let data = get_server_data().await?;
                println!("Client received: {}", data);
                text.set(data.clone());
                post_server_data(data).await?;
                Ok(())
            },
            "Run a server function!"
        }
        "Server said: {text}"

        // Async clipboard test
        button {
            onclick: move |_| async move {
                let counter_text = format!("Counter: {}", count());
                match clipboard.set(counter_text).await {
                    Ok(()) => {
                        tracing::info!("✅ Async copied to clipboard!");
                    }
                    Err(e) => {
                        tracing::error!("❌ Async clipboard failed: {:?}", e);
                    }
                }
            },
            "Copy Counter to Clipboard (Async)"
        }

        button {
            onclick: move |_| async move {
                match clipboard.get().await {
                    Ok(content) => {
                        tracing::info!("✅ Async read from clipboard: {}", content);
                        text.set(format!("Clipboard: {}", content));
                    }
                    Err(e) => {
                        tracing::error!("❌ Async clipboard read failed: {:?}", e);
                    }
                }
            },
            "Read from Clipboard (Async)"
        }
    }
}

#[server]
async fn post_server_data(data: String) -> ServerFnResult {
    println!("Server received: {}", data);

    Ok(())
}

#[server]
async fn get_server_data() -> ServerFnResult<String> {
    Ok(reqwest::get("https://httpbin.org/ip").await?.text().await?)
}

fn main() {
    dioxus::launch(app);
}
