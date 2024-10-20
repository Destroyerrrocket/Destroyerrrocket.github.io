use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

mod components;
mod sections;
mod utils;

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

#[cfg(not(target_family = "wasm"))]
fn generate_all_route_files() {
    let docs_dir = std::env::current_dir().unwrap().join("docs");
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

#[cfg(not(target_family = "wasm"))]
fn main() {
    config_logger();
    generate_all_route_files();
}

#[cfg(target_family = "wasm")]
fn main() {
    // Init logger
    config_logger();
    launch(App);
}

#[component]
fn MainSectionDisplayed(route: Vec<String>, current_section: sections::ActiveSection) -> Element {
    match current_section {
        sections::ActiveSection::AboutMe => rsx! {
            main { sections::about_me::AboutMe {} }
        },
        sections::ActiveSection::PasswordGenerator => rsx! {
            main { sections::password_generator::PasswordGenerator {} }
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
            "Password Generator",
            sections::ActiveSection::PasswordGenerator,
            current_section,
        ),
    ];

    rsx! {
        components::title_bar::TitleBar { entries: titleBarEntries }
        MainSectionDisplayed { route, current_section }
    }
}
