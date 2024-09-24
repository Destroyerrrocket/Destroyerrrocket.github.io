use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

mod components;
mod sections;

fn main() {
    // Init logger
    dioxus_logger::init(Level::TRACE).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
fn MainSectionDisplayed() -> Element {
    let active_section: Signal<sections::ActiveSection> =
        consume_context::<Signal<sections::ActiveSection>>();
    match active_section() {
        sections::ActiveSection::AboutMe => rsx! {
            sections::about_me::AboutMe {}
        },
        sections::ActiveSection::HelloWorld => rsx! {
            sections::hello_world::HelloWorld {}
        },
    }
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(sections::ActiveSection::HelloWorld));

    let titleBarEntries = vec![
        components::title_bar::TitleEntry {
            name: "About Me",
            section: sections::ActiveSection::AboutMe,
        },
        components::title_bar::TitleEntry {
            name: "Hello World",
            section: sections::ActiveSection::HelloWorld,
        },
    ];
    rsx! {
        components::title_bar::TitleBar { entries: titleBarEntries }
        MainSectionDisplayed {}
    }
}
