use crate::components::lets_connect::LetsConnectAddresses;
use crate::components::skill_bars::{SkillBarProps, SkillBars};
use dioxus::prelude::*;
use dioxus_logger::tracing::*;

#[component]
pub fn AboutMe() -> Element {
    info!("Creating about me");

    pub const ME_IMG: manganis::ImageAsset = manganis::mg!(image("./raw_assets/img/me.jpeg")
        .size(384, 384)
        .format(ImageType::Avif)
        .preload());

    let skills = vec![
        SkillBarProps::new("C++", 100),
        SkillBarProps::new("Rust", 95),
        SkillBarProps::new("Python", 70),
        SkillBarProps::new("C#", 50),
        SkillBarProps::new("HTML/CSS", 100),
    ];

    rsx! {
        div { class: "relative bg-cover bg-center bg-no-repeat py-8",
            div { class: "absolute inset-0 z-20 bg-gradient-to-r from-hero-gradient-from to-hero-gradient-to bg-cover bg-center bg-no-repeat" }
            div { class: "container relative z-30 pt-20 pb-12 sm:pt-56 sm:pb-48 lg:pt-64 lg:pb-48",
                div { class: "flex flex-col items-center justify-center lg:flex-row",
                    div { class: "rounded-full border-8 border-primary shadow-xl",
                        img {
                            src: "{ME_IMG}",
                            loading: "lazy",
                            decoding: "async",
                            alt: "Pol Marcet Sardà's photo",
                            class: "h-48 rounded-full sm:h-56"
                        }
                    }
                    div { class: "pt-8 sm:pt-10 lg:pl-8 lg:pt-0",
                        h1 { class: "text-center font-header text-4xl text-white-text sm:text-left sm:text-5xl md:text-6xl",
                            "Hello, I'm Pol Marcet Sardà!"
                        }
                        LetsConnectAddresses {}
                    }
                }
            }
        }

        div { class: "bg-grey-50", id: "about",
            div { class: "container flex flex-col items-center py-16 md:py-20 lg:flex-row",
                div { class: "w-full text-center sm:w-3/4 lg:w-3/5 lg:text-left",
                    h2 { class: "font-header text-4xl font-semibold uppercase text-primary sm:text-5xl lg:text-6xl",
                        "Who am I?"
                    }
                    h3 { class: "pt-6 font-header text-xl font-medium text-secondary sm:text-2xl lg:text-3xl",
                        "I'm Pol, a C++ software engineer and software architect."
                    }
                    p { class: "pt-6 font-body leading-relaxed text-primary",
                        "I've been writting code since I was 9. I've always been "
                        "passionate about technology and how it can be used to "
                        "solve complex problems."
                    }
                    p { class: "pt-6 font-body leading-relaxed text-primary",
                        "I've worked on a wide range of projects; It all started "
                        "with videogame development when I was a kid, followed "
                        "by jumps through web fronted/backend development, and "
                        "ending on high performance and compiler programming. "
                        "Since I discovered this field, I knew I had discovered "
                        "my passion, and long story short, I've been working on "
                        "it since!"
                    }
                }
                SkillBars { skills }
            }
        }
        div { class: "container py-16 md:py-20", id: "work",
            h2 { class: "text-center font-header text-4xl font-semibold uppercase text-primary sm:text-5xl lg:text-6xl",
                "My engineering timeline"
            }
            h3 { class: "pt-6 text-center font-header text-xl font-medium text-black sm:text-2xl lg:text-3xl",
                "Here's what I've done so far"
            }
            div { class: "relative mx-auto mt-12 flex w-full flex-col lg:w-2/3",
                span { class: "left-2/5 absolute inset-y-0 ml-10 hidden w-0.5 bg-grey-40 md:block" }
                div { class: "mt-8 flex flex-col text-center md:flex-row md:text-left",
                    div { class: "md:w-2/5",
                        div { class: "flex justify-center md:justify-start",
                            span { class: "shrink-0",
                                img {
                                    src: "/img/keysight_logo.svg",
                                    alt: "Keysight Technologies logo",
                                    loading: "lazy",
                                    decoding: "async",
                                    class: "h-auto w-32"
                                }
                            }
                            div { class: "relative ml-3 hidden w-full md:block",
                                span { class: "absolute inset-x-0 top-1/2 h-0.5 -translate-y-1/2 transform bg-grey-70" }
                            }
                        }
                    }
                    div { class: "md:w-3/5",
                        div { class: "relative flex md:pl-18",
                            span { class: "absolute left-8 top-1 hidden h-4 w-4 rounded-full border-2 border-grey-40 bg-white md:block" }
                            div { class: "mt-1 flex",
                                i { class: "bx bxs-right-arrow hidden text-primary md:block" }
                                div { class: "md:-mt-1 md:pl-8",
                                    span { class: "block font-body font-bold text-grey-20",
                                        "07/2024 - Present"
                                    }
                                    span { class: "block pt-2 font-header text-xl font-bold uppercase text-primary",
                                        "Software Architect"
                                    }
                                    div { class: "pt-2",
                                        span { class: "block font-body text-black hidden",
                                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum mattis felis vitae risus pulvinar tincidunt. Nam ac venenatis enim."
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "mt-8 flex flex-col text-center md:flex-row md:text-left",
                    div { class: "md:w-2/5",
                        div { class: "flex justify-center md:justify-start",
                            span { class: "shrink-0",
                                img {
                                    src: "/img/keysight_logo.svg",
                                    alt: "Keysight Technologies logo",
                                    loading: "lazy",
                                    decoding: "async",
                                    class: "h-auto w-32"
                                }
                            }
                            div { class: "relative ml-3 hidden w-full md:block",
                                span { class: "absolute inset-x-0 top-1/2 h-0.5 -translate-y-1/2 transform bg-grey-70" }
                            }
                        }
                    }
                    div { class: "md:w-3/5",
                        div { class: "relative flex md:pl-18",
                            span { class: "absolute left-8 top-1 hidden h-4 w-4 rounded-full border-2 border-grey-40 bg-white md:block" }
                            div { class: "mt-1 flex",
                                i { class: "bx bxs-right-arrow hidden text-primary md:block" }
                                div { class: "md:-mt-1 md:pl-8",
                                    span { class: "block font-body font-bold text-grey-20",
                                        "06/2022 - 07/2024"
                                    }
                                    span { class: "block pt-2 font-header text-xl font-bold uppercase text-primary",
                                        "Software Engineer"
                                    }
                                    div { class: "pt-2",
                                        span { class: "block font-body text-black hidden",
                                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n                  Vestibulum mattis felis vitae risus pulvinar tincidunt. Nam ac\n                  venenatis enim."
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "mt-8 flex hidden flex-col text-center md:flex-row md:text-left",
                    div { class: "md:w-2/5",
                        div { class: "flex justify-center md:justify-start",
                            span { class: "shrink-0",
                                img {
                                    src: "/img/keysight_logo.svg",
                                    alt: "Keysight Technologies logo",
                                    loading: "lazy",
                                    decoding: "async",
                                    class: "h-auto w-32"
                                }
                            }
                            div { class: "relative ml-3 hidden w-full md:block",
                                span { class: "absolute inset-x-0 top-1/2 h-0.5 -translate-y-1/2 transform bg-grey-70" }
                            }
                        }
                    }
                    div { class: "md:w-3/5",
                        div { class: "relative flex md:pl-18",
                            span { class: "absolute left-8 top-1 hidden h-4 w-4 rounded-full border-2 border-grey-40 bg-white md:block" }
                            div { class: "mt-1 flex",
                                i { class: "bx bxs-right-arrow hidden text-primary md:block" }
                                div { class: "md:-mt-1 md:pl-8",
                                    span { class: "block font-body font-bold text-grey-20",
                                        "October 2019 - Feb 2021"
                                    }
                                    span { class: "block pt-2 font-header text-xl font-bold uppercase text-primary",
                                        "DevOps Engineer"
                                    }
                                    div { class: "pt-2",
                                        span { class: "block font-body text-black",
                                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n                  Vestibulum mattis felis vitae risus pulvinar tincidunt. Nam ac\n                  venenatis enim."
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/*
Work experience timeline I'm working on.

*/
