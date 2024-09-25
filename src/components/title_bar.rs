use dioxus::prelude::*;

use crate::sections;

#[derive(Clone, Debug)]
pub struct TitleEntry {
    pub name: &'static str,
    pub section: sections::ActiveSection,
}

#[component]
pub fn TitleBar(entries: ReadOnlySignal<Vec<TitleEntry>>) -> Element {
    use_context_provider(|| Signal::new(MobileBurgerMenuShown(false)));

    rsx! {
        // Desktop top bar
        DesktopTitleBar { entries }
        // Mobile top bar
        ShowMobileBurgerMenu {}
        MobileBurgerMenu { entries }
    }
}

#[derive(Clone, Debug)]
struct MobileBurgerMenuShown(bool);

#[component]
fn ShowMobileBurgerMenu() -> Element {
    let mut mobile_burger_menu_shown: Signal<MobileBurgerMenuShown> =
        consume_context::<Signal<MobileBurgerMenuShown>>();

    let classes = match mobile_burger_menu_shown() {
        MobileBurgerMenuShown(false) => "",
        MobileBurgerMenuShown(true) => "is-active",
    };
    rsx! {
        div { class: "z-71 top-2 right-20 fixed lg:hidden",
            button {
                aria_label: "Website menu",
                class: "hamburger absolute mt-4 mr-4 hamburger--spin {classes}",
                onclick: move |_event| {
                    let MobileBurgerMenuShown(is_shown) = mobile_burger_menu_shown();
                    *mobile_burger_menu_shown.write() = MobileBurgerMenuShown(!is_shown);
                },
                span { class: "hamburger-box",
                    span { class: "bg-white-text hamburger-inner" }
                }
            }
        }
    }
}

#[component]
fn MobileBurgerButton(entry: ReadOnlySignal<TitleEntry>) -> Element {
    let mut active_section: Signal<sections::ActiveSection> =
        consume_context::<Signal<sections::ActiveSection>>();
    let mut mobile_burger_menu_shown: Signal<MobileBurgerMenuShown> =
        consume_context::<Signal<MobileBurgerMenuShown>>();
    let TitleEntry { name, section } = *entry.read();
    rsx! {
        li { class: "py-2",
            a {
                class: "pt-0.5 font-header font-semibold uppercase text-white-text",
                onclick: move |_event| {
                    *active_section.write() = section;
                    *mobile_burger_menu_shown.write() = MobileBurgerMenuShown(false);
                },
                "{name}"
            }
        }
    }
}

#[component]
fn MobileBurgerMenu(entries: ReadOnlySignal<Vec<TitleEntry>>) -> Element {
    let mobile_burger_menu_shown: Signal<MobileBurgerMenuShown> =
        consume_context::<Signal<MobileBurgerMenuShown>>();

    let classes = match mobile_burger_menu_shown() {
        MobileBurgerMenuShown(false) => "pointer-events-none opacity-0",
        MobileBurgerMenuShown(true) => "pointer-events-auto opacity-100",
    };

    rsx! {
        div { class: "{classes} fixed inset-0 z-70 min-h-screen bg-black bg-opacity-70 transition-opacity lg:hidden",
            div { class: "absolute right-0 min-h-screen w-2/3 bg-primary py-4 px-8 shadow md:w-1/3",
                ul { class: "mt-8 flex flex-col",
                    {
                    entries.read().iter().map(|entry| {
                        let entryClone = entry.clone();
                        rsx! {MobileBurgerButton { entry: entryClone }}
                    })
                    }
                }
            }
        }
    }
}

#[component]
fn DesktopTitleButton(entry: ReadOnlySignal<TitleEntry>) -> Element {
    let TitleEntry { name, section } = *entry.read();

    let mut active_section: Signal<sections::ActiveSection> =
        consume_context::<Signal<sections::ActiveSection>>();

    let selected_section_classes = if active_section() == section {
        "bg-white-text"
    } else {
        ""
    };

    rsx! {
        li { class: "group pl-6",
            a {
                class: "cursor-pointer pt-0.5 font-header font-semibold uppercase text-white-text",
                onclick: move |_event| {
                    *active_section.write() = section;
                },
                "{name}"
            }
            span { class: "{selected_section_classes} block h-0.5 w-full bg-transparent transition-colors group-hover:bg-white-text" }
        }
    }
}

#[component]
fn DesktopTitleBar(entries: ReadOnlySignal<Vec<TitleEntry>>) -> Element {
    rsx! {
        div { class: "z-70 bg-primary fixed w-full flex items-center justify-between",
            div { class: "hidden mt-1 mb-1 lg:block",
                ul { class: "flex items-center",
                    {
                        entries.read().iter().map(|entry| {
                            let entryClone = entry.clone();
                            rsx! {DesktopTitleButton { entry: entryClone }}
                        })
                    }
                }
            }
        }
    }
}
