use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct LetsConnectAddressProps {
    #[props(into)]
    pub link: String,
    #[props(into)]
    pub icon: String,
    // This value will be used to add a padding to the left of all except the first element. Don't set it manually.
    #[props(default)]
    pub is_first: bool,
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct LetsConnectAddressesProps {
    pub addresses: Option<Vec<LetsConnectAddressProps>>,
}

impl LetsConnectAddressProps {
    pub fn new(link: impl Into<String>, icon: impl Into<String>) -> Self {
        Self {
            link: link.into(),
            icon: icon.into(),
            is_first: false,
        }
    }
}

impl Default for LetsConnectAddressesProps {
    fn default() -> Self {
        Self {
            addresses: Some(vec![
                LetsConnectAddressProps::new("mailto:pol@marcet.biz", "bxl-gmail"),
                LetsConnectAddressProps::new(
                    "https://www.linkedin.com/in/pol-marcet-sard%C3%A0-a40a5817a/",
                    "bxl-linkedin",
                ),
            ]),
        }
    }
}

#[allow(non_snake_case)]
fn LetsConnectAddress(
    LetsConnectAddressProps {
        link,
        icon,
        is_first,
    }: LetsConnectAddressProps,
) -> Element {
    rsx! {
        a {
            href: "{link}",
            target: "_blank",
            class: if is_first { "" } else { "pl-4" },
            i { class: "bx {icon} text-2xl text-white hover:text-tertiary" }
        }
    }
}

#[allow(non_snake_case)]
pub fn LetsConnectAddresses(
    LetsConnectAddressesProps { addresses }: LetsConnectAddressesProps,
) -> Element {
    let addresses =
        addresses.unwrap_or_else(|| LetsConnectAddressesProps::default().addresses.unwrap());

    let mut is_first_entry = true;

    rsx! {
        div { class: "flex flex-col justify-center pt-3 sm:flex-row sm:pt-5 lg:justify-start",
            div { class: "flex items-center justify-center pl-0 sm:justify-start md:pl-1",
                p { class: "font-body text-lg uppercase text-white leading-none",
                    "Let's connect"
                }
                div { class: "hidden sm:block",
                    i { class: "bx bx-chevron-right text-3xl text-tertiary" }
                }
            }
            div { class: "flex items-center justify-center pt-5 sm:justify-start sm:pt-0",
                for LetsConnectAddressProps { link , icon , .. } in addresses {
                    LetsConnectAddress { link, icon, is_first: is_first_entry }
                    {is_first_entry = false;}
                }
            }
        }
    }
}
