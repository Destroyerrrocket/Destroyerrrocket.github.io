use dioxus::prelude::*;

use crate::sections;

#[derive(Clone, Debug)]
pub struct TitleEntry {
    pub name: &'static str,
    pub section: sections::ActiveSection,
}

#[component]
pub fn TitleBar(entries: ReadOnlySignal<Vec<TitleEntry>>) -> Element {
    use_context_provider(|| Signal::new(MobileBurgerMenuShown::False));

    rsx! {
        // Desktop top bar
        DesktopTitleBar { entries }
        // Mobile top bar
        ShowMobileBurgerMenu {}
        MobileBurgerMenu { entries }
    }
}

#[component]
fn buttonBar() -> Element {
    let mut active_section: Signal<sections::ActiveSection> =
        consume_context::<Signal<sections::ActiveSection>>();

    rsx! {
        button {
            onclick: move |_event| {
                *active_section.write() = sections::ActiveSection::AboutMe;
            },
            "About Me"
        }
    }
}

#[derive(Clone, Debug)]
enum MobileBurgerMenuShown {
    True,
    False,
}

#[component]
fn ShowMobileBurgerMenu() -> Element {
    let mut mobile_burger_menu_shown: Signal<MobileBurgerMenuShown> =
        consume_context::<Signal<MobileBurgerMenuShown>>();

    let classes = match mobile_burger_menu_shown() {
        MobileBurgerMenuShown::False => "",
        MobileBurgerMenuShown::True => "invisible",
    };
    rsx! {
        div { class: "w-full min-h-screen fixed lg:hidden",
            button {
                class: "{classes} absolute top-0 right-4 mt-4 mr-4",
                onclick: move |_event| { *mobile_burger_menu_shown.write() = MobileBurgerMenuShown::True },
                i { class: "bx bx-menu text-4xl text-white" }
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
                class: "pt-0.5 font-header font-semibold uppercase text-white",
                onclick: move |_event| {
                    *active_section.write() = section;
                    *mobile_burger_menu_shown.write() = MobileBurgerMenuShown::False;
                },
                "{name}"
            }
        }
    }
}

#[component]
fn MobileBurgerMenu(entries: ReadOnlySignal<Vec<TitleEntry>>) -> Element {
    let mut mobile_burger_menu_shown: Signal<MobileBurgerMenuShown> =
        consume_context::<Signal<MobileBurgerMenuShown>>();

    let classes = match mobile_burger_menu_shown() {
        MobileBurgerMenuShown::False => "pointer-events-none opacity-0",
        MobileBurgerMenuShown::True => "pointer-events-auto opacity-100",
    };

    rsx! {
        div { class: "{classes} fixed inset-0 z-70 min-h-screen bg-black bg-opacity-70 transition-opacity lg:hidden",
            div { class: "absolute right-0 min-h-screen w-2/3 bg-primary py-4 px-8 shadow md:w-1/3",
                button {
                    class: "absolute top-0 right-0 mt-4 mr-4",
                    onclick: move |_event| { *mobile_burger_menu_shown.write() = MobileBurgerMenuShown::False },

                    img {
                        src: "img/icon-close.svg",
                        alt: "",
                        class: "h-10 w-auto"
                    }
                }
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
    let mut active_section: Signal<sections::ActiveSection> =
        consume_context::<Signal<sections::ActiveSection>>();
    let TitleEntry { name, section } = *entry.read();
    rsx! {
        li { class: "group pl-6",
            a {
                class: "cursor-pointer pt-0.5 font-header font-semibold uppercase text-white",
                onclick: move |_event| {
                    *active_section.write() = section;
                },
                "{name}"
            }
            span { class: "block h-0.5 w-full bg-transparent transition-colors group-hover:bg-white" }
        }
    }
}

#[component]
fn DesktopTitleBar(entries: ReadOnlySignal<Vec<TitleEntry>>) -> Element {
    rsx! {
        div { class: "bg-slate-800 fixed w-full flex items-center justify-between",
            div { class: "hidden lg:block",
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
