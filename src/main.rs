use std::env;

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

#[cfg(target_family = "wasm")]
#[inline]
fn is_wasm() -> bool {
    true
}

#[cfg(target_family = "unix")]
#[inline]
fn is_wasm() -> bool {
    false
}

#[cfg(target_family = "windows")]
#[inline]
fn is_wasm() -> bool {
    false
}

fn generate_all_route_files() {
    let docs_dir = env::current_dir().unwrap().join("docs");
    assert!(
        docs_dir.exists(),
        "docs directory not found. Please run `./build.sh`"
    );

    let index_file = docs_dir.join("index.html");
    assert!(
        index_file.exists(),
        "index.html not found. Please run `./build.sh`"
    );

    for route in sections::ActiveSection::all_routes() {
        let route = route.join("/");
        let file = docs_dir.join(route);
        let parent_dir = file.parent().unwrap();
        let file_str = file.to_str().unwrap();
        info!("file: {file_str}");
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir).expect("failed to create parent dir for a file");
        } else {
            assert!(parent_dir.is_dir());
        }

        std::fs::copy(&index_file, file).expect("failed to copy index.html to new route");
    }
}

fn main() {
    // Init logger
    config_logger();
    if is_wasm() {
        launch(App);
    } else {
        generate_all_route_files();
    }
}

#[component]
fn MainSectionDisplayed(route: Vec<String>, current_section: sections::ActiveSection) -> Element {
    match current_section {
        sections::ActiveSection::AboutMe => rsx! {
            main { sections::about_me::AboutMe {} }
        },
        sections::ActiveSection::HelloWorld => rsx! {
            main { sections::hello_world::HelloWorld {} }
        },
    }
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    //#[route("/:..route?:..query")]
    //NavBar {route: Vec<String>, query: String},

    #[route("/:..route")]
    NavBar {route: Vec<String>},
}

#[component]
pub fn App() -> Element {
    info!("Starting app");
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
