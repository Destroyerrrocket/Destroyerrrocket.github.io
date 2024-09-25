use dioxus::prelude::*;
use dioxus_logger::tracing::*;

#[component]
pub fn AboutMe() -> Element {
    trace!("Creating about me");
    // New
    rsx! {
        div { class: "w-full h-screen bg-secondary text-white-text flex items-center justify-center",
            "I'm Pol Marcet Sardà!"
        }
    }
}
