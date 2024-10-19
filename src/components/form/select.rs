use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SelectProps {
    pub source: Signal<String>,
    pub values: Vec<String>,
    #[props(into, default)]
    pub default: Option<String>,
    #[props(into, default)]
    pub id: Option<String>,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let SelectProps {
        mut source,
        values,
        default,
        id,
    } = props;
    let selected = source.read();

    rsx! {
        select {
            id,
            onchange: move |event| source.set(event.value()),
            class: "block w-full px-4 py-2 mt-2 text-gray-700 bg-white border border-gray-300 rounded-md dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600 focus:border-blue-500 dark:focus:border-blue-500 focus:outline-none focus:ring",
            for value in values {
                option {
                    selected: selected.clone() == value,
                    initial_selected: default.as_ref() == Some(&value),
                    "{value}"
                }
            }
        }
    }
}
