use dioxus::logger::tracing::{info, Level};
use dioxus::prelude::*;

use crate::sections::ActiveSection;

mod components;
mod sections;
mod utils;

#[cfg(debug_assertions)]
fn config_logger() {
    dioxus::logger::init(Level::DEBUG).expect("failed to init logger");
    info!("starting app");
}

#[cfg(not(debug_assertions))]
fn config_logger() {
    dioxus::logger::init(Level::WARN).expect("failed to init logger");
    info!("starting app");
}

#[cfg(feature = "generate_htmls")]
fn generate_blog_files(docs_dir: &std::path::Path, index_file: &std::path::PathBuf) {
    let blog_dir = docs_dir.join("blog");
    for entry in std::fs::read_dir("assets/blog").unwrap() {
        let origin_path_year = entry.unwrap().path();
        if !origin_path_year.is_dir() {
            continue;
        }

        let year_dir = blog_dir.join(origin_path_year.file_name().unwrap());
        if !year_dir.exists() {
            std::fs::create_dir_all(year_dir.clone())
                .expect("failed to create year_dir dir for a post");
        } else {
            assert!(year_dir.is_dir());
        }
        for entry in origin_path_year.read_dir().unwrap() {
            let origin_path_month = entry.unwrap().path();
            if !origin_path_month.is_dir() {
                continue;
            }

            let month_dir = year_dir.join(origin_path_month.file_name().unwrap());
            if !month_dir.exists() {
                std::fs::create_dir_all(month_dir.clone())
                    .expect("failed to create month_dir dir for a post");
            } else {
                assert!(month_dir.is_dir());
            }
            for entry in origin_path_month.read_dir().unwrap() {
                let origin_path_day = entry.unwrap().path();
                if !origin_path_day.is_dir() {
                    continue;
                }

                let day_dir = month_dir.join(origin_path_day.file_name().unwrap());
                if !day_dir.exists() {
                    std::fs::create_dir_all(day_dir.clone())
                        .expect("failed to create day_dir dir for a post");
                } else {
                    assert!(day_dir.is_dir());
                }

                let file = day_dir.join("index.html");
                let file_str = file.to_str().unwrap();
                info!("file: {file_str}");

                std::fs::copy(index_file, file).expect("failed to copy index.html to new route");
            }
        }
    }
}

#[cfg(feature = "generate_htmls")]
fn generate_all_route_files() {
    let docs_dir = std::env::current_dir()
        .unwrap()
        .join(std::env::args().collect::<Vec<_>>().get(1).unwrap());
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
        let file = docs_dir.join(&route);
        let parent_dir = file.parent().unwrap();
        let file_str = file.to_str().unwrap();
        info!("file: {file_str}");
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir).expect("failed to create parent dir for a file");
        } else {
            assert!(parent_dir.is_dir());
        }

        let static_version = std::env::current_dir()
            .unwrap()
            .join("static")
            .join(route)
            .join("index.html");

        if static_version.is_file() {
            info!(
                "found static version at: {}",
                static_version.to_str().unwrap()
            );
            std::fs::copy(static_version, file)
                .expect("failed to copy static version of page to new route");
        } else {
            std::fs::copy(&index_file, file).expect("failed to copy index.html to new route");
        }
    }

    // Special case for posts
    generate_blog_files(&docs_dir, &index_file);

    // Special case for the root
    let static_version = std::env::current_dir()
        .unwrap()
        .join("static")
        .join("index.html");

    if static_version.is_file() {
        info!(
            "found static version at: {}",
            static_version.to_str().unwrap()
        );
        std::fs::copy(static_version, index_file)
            .expect("failed to copy static version of page to new route");
    }
}

#[cfg(feature = "generate_htmls")]
fn main() {
    config_logger();
    generate_all_route_files();
}

#[cfg(any(feature = "server", feature = "web"))]
fn main() {
    // Init logger
    config_logger();
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            ServeConfig::builder()
                // turn on incremental site generation with the .incremental() method
                .incremental(IncrementalRendererConfig::new())
                .build()
                .unwrap()
        })
        .launch(App);
    launch(App);
}

#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(ActiveSection::all_static_routes()
        .into_iter()
        .map(|r| format!("/{}", r.join("/")))
        .chain(vec!["/".to_string()].into_iter())
        .collect())
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
        sections::ActiveSection::Blog(blog_date) => rsx! {
            main {
                sections::blog::Blog { blog_date }
            }
        },
    }
}

#[derive(Debug, Routable, Clone)]
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
        document::Stylesheet { href: asset!("assets/css/tailwind.css") }
        document::Stylesheet { href: asset!("assets/css/hamburgers-min.css") }
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
        components::title_bar::TitleEntry::new(
            "Blog",
            sections::ActiveSection::Blog(None),
            current_section,
        ),
    ];

    rsx! {
        components::title_bar::TitleBar { entries: titleBarEntries }
        MainSectionDisplayed { route, current_section }
    }
}
