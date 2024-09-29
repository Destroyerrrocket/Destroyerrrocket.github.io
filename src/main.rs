use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

mod components;
mod sections;

#[cfg(debug_assertions)]
fn config_logger() {
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    info!("starting app");
}

#[cfg(not(debug_assertions))]
fn config_logger() {
    dioxus_logger::init(Level::WARN).expect("failed to init logger");
    info!("starting app");
}

fn main() {
    // Init logger
    config_logger();
    launch(App);
}

#[component]
fn MainSectionDisplayed(route: Vec<String>, current_section: sections::ActiveSection) -> Element {
    match current_section {
        sections::ActiveSection::AboutMe => rsx! {
            sections::about_me::AboutMe {}
        },
        sections::ActiveSection::HelloWorld => rsx! {
            sections::hello_world::HelloWorld {}
        },
    }
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[route("/:..route")]
    NavBar {route: Vec<String>},
}

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn NavBar(route: Vec<String>) -> Element {
    let current_section = sections::ActiveSection::try_from(route.as_slice())
        .unwrap_or(sections::ActiveSection::AboutMe);

    info!("current_section: {:?} [{:?}]", current_section, route);

    let titleBarEntries = vec![
        components::title_bar::TitleEntry::new(
            "About Me",
            sections::ActiveSection::AboutMe,
            current_section,
        ),
        components::title_bar::TitleEntry::new(
            "Hello World",
            sections::ActiveSection::HelloWorld,
            current_section,
        ),
    ];

    rsx! {
        components::title_bar::TitleBar { entries: titleBarEntries }
        MainSectionDisplayed { route, current_section }
    }
}
