use dioxus::prelude::*;
use dioxus_logger::tracing::*;

#[component]
pub fn HelloWorld() -> Element {
    trace!("Creating hello world");
    // New
    rsx! {
        div { class: "w-full h-screen bg-gray-900 text-white flex items-center justify-center",
            "Hello, world!"
        }
    }
}
