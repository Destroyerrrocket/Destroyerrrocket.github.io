#[cfg(feature = "web")]
pub async fn set_clipboard(text: String) {
    let ctx = web_sys::window().unwrap().navigator().clipboard();
    let res = wasm_bindgen_futures::JsFuture::from(ctx.write_text(text.as_str())).await;
    if let Err(err) = res {
        use dioxus::logger::tracing::*;
        error!("Failed to write to clipboard: {:?}", err);
    }
}

#[cfg(not(feature = "web"))]
pub async fn set_clipboard(_text: String) {}
