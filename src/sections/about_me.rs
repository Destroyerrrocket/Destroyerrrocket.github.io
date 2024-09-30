use crate::components::lets_connect::LetsConnectAddresses;
use crate::components::skill_bars::{SkillBarProps, SkillBars};
use dioxus::prelude::*;
use dioxus_logger::tracing::*;

#[component]
pub fn AboutMe() -> Element {
    trace!("Creating about me");

    pub const ME_IMG: manganis::ImageAsset = manganis::mg!(image("./raw_assets/img/me.jpeg")
        // Manganis uses the builder pattern inside the macro. You can set the image size in pixels at compile time to send the smallest possible image to the client
        .size(512, 512)
        // You can also convert the image to a web friendly format at compile time. This can make your images significantly smaller
        .format(ImageType::Avif)
        // You can even tell manganis to preload the image so it's ready to be displayed as soon as it's needed
        .preload());

    let skills = vec![
        SkillBarProps::new("C++", 100),
        SkillBarProps::new("Rust", 95),
        SkillBarProps::new("Python", 70),
        SkillBarProps::new("C#", 50),
        SkillBarProps::new("HTML/CSS", 100),
        SkillBarProps::new("Javascript", 1),
    ];

    rsx! {
        div { class: "relative bg-cover bg-center bg-no-repeat py-8",
            div { class: "absolute inset-0 z-20 bg-gradient-to-r from-hero-gradient-from to-hero-gradient-to bg-cover bg-center bg-no-repeat" }
            div { class: "container relative z-30 pt-20 pb-12 sm:pt-56 sm:pb-48 lg:pt-64 lg:pb-48",
                div { class: "flex flex-col items-center justify-center lg:flex-row",
                    div { class: "rounded-full border-8 border-primary shadow-xl",
                        img {
                            src: "{ME_IMG}",
                            alt: "Pol Marcet Sardà's photo",
                            class: "h-48 rounded-full sm:h-56"
                        }
                    }
                    div { class: "pt-8 sm:pt-10 lg:pl-8 lg:pt-0",
                        h1 { class: "text-center font-header text-4xl text-white sm:text-left sm:text-5xl md:text-6xl",
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
                    h4 { class: "pt-6 font-header text-xl font-medium text-black sm:text-2xl lg:text-3xl",
                        "I'm Pol, a C++ software engineer and software architect."
                    }
                    p { class: "pt-6 font-body leading-relaxed text-grey-20",
                        "I've been writting code since I was 9. I've always been "
                        "passionate about technology and how it can be used to "
                        "solve complex problems."
                    }
                    p { class: "pt-6 font-body leading-relaxed text-grey-20",
                        "I've worked on a wide range of projects; It all started "
                        "with videogame development when I was a kid, followed "
                        "by jumps through web fronted/backend development, and "
                        "ending on high performance and compiler programming. "
                        "Since I discovered this field, I knew I had discovered "
                        "my field."
                    }
                }
                SkillBars { skills }
            }
        }
    }
}

/*
Work experience timeline I'm working on.
div { class: "container py-16 md:py-20", id: "work",
    h2 { class: "text-center font-header text-4xl font-semibold uppercase text-primary sm:text-5xl lg:text-6xl",
        "\n    My work experience\n  "
    }
    h3 { class: "pt-6 text-center font-header text-xl font-medium text-black sm:text-2xl lg:text-3xl",
        "\n    Here's what I did before freelancing\n  "
    }
    div { class: "relative mx-auto mt-12 flex w-full flex-col lg:w-2/3",
        span { class: "left-2/5 absolute inset-y-0 ml-10 hidden w-0.5 bg-grey-40 md:block" }
        div { class: "mt-8 flex flex-col text-center md:flex-row md:text-left",
            div { class: "md:w-2/5",
                div { class: "flex justify-center md:justify-start",
                    span { class: "shrink-0",
                        img {
                            src: "/assets/img/logo-spotify.svg",
                            alt: "company logo",
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
                            span { class: "block font-body font-bold text-grey-40",
                                "Apr 2015 - Mar 2018"
                            }
                            span { class: "block pt-2 font-header text-xl font-bold uppercase text-primary",
                                "Frontend Developer"
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
        div { class: "mt-8 flex flex-col text-center md:flex-row md:text-left",
            div { class: "md:w-2/5",
                div { class: "flex justify-center md:justify-start",
                    span { class: "shrink-0",
                        img {
                            alt: "company logo",
                            src: "/assets/img/logo-microsoft.svg",
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
                            span { class: "block font-body font-bold text-grey-40",
                                "Mar 2018 - September 2019"
                            }
                            span { class: "block pt-2 font-header text-xl font-bold uppercase text-primary",
                                "Software Engineer"
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
        div { class: "mt-8 flex flex-col text-center md:flex-row md:text-left",
            div { class: "md:w-2/5",
                div { class: "flex justify-center md:justify-start",
                    span { class: "shrink-0",
                        img {
                            alt: "company logo",
                            src: "/assets/img/logo-fedex.svg",
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
                            span { class: "block font-body font-bold text-grey-40",
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
*/
