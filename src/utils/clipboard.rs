pub fn set_clipboard(text: &str) {
    let ctx = web_sys::window().unwrap().navigator().clipboard();
    let _ = ctx.write_text(text);
}
