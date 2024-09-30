use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SkillBarProps {
    #[props(into)]
    pub name: String,
    pub percentage: u8,
}

impl SkillBarProps {
    pub fn new(name: impl Into<String>, percentage: u8) -> Self {
        Self {
            name: name.into(),
            percentage,
        }
    }
}

#[component]
fn SkillBar(props: SkillBarProps) -> Element {
    rsx! {
        div {
            div { class: "flex items-end justify-between",
                h4 { class: "font-body font-semibold uppercase text-black", "{props.name}" }
                h3 { class: "font-body text-3xl font-bold text-primary", "{props.percentage}%" }
            }
            div { class: "mt-2 h-3 w-full rounded-full bg-quaternary",
                div {
                    style: "width: {props.percentage}%",
                    class: "h-3 rounded-full bg-primary"
                }
            }
        }
    }
}

#[component]
pub fn SkillBars(skills: Vec<SkillBarProps>) -> Element {
    rsx! {
        div { class: "w-full pl-0 pt-10 sm:w-3/4 lg:w-2/5 lg:pl-12 lg:pt-0",
            for SkillBarProps { name , percentage } in skills {
                SkillBar { name, percentage }
            }
        }
    }
}
