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

        // Simple clipboard test
        button {
            onclick: move |_| {
                let counter_text = format!("Counter: {}", count());
                match clipboard.set(counter_text) {
                    Ok(()) => {
                        tracing::info!("✅ Copied to clipboard!");
                    }
                    Err(e) => {
                        tracing::error!("❌ Clipboard failed: {:?}", e);
                    }
                }
            },
            "Copy Counter to Clipboard"
        }

        button {
            onclick: move |_| {
                match clipboard.get() {
                    Ok(content) => {
                        tracing::info!("✅ Read from clipboard: {}", content);
                        text.set(format!("Clipboard: {}", content));
                    }
                    Err(e) => {
                        tracing::error!("❌ Clipboard read failed: {:?}", e);
                    }
                }
            },
            "Read from Clipboard"
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
