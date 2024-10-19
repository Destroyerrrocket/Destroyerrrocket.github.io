use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ReadOnlyInputTextProps {
    pub source: Signal<String>,
    #[props(into, default)]
    pub placeholder: String,
    #[props(into, default)]
    pub id: Option<String>,
}

#[component]
pub fn ReadOnlyInputText(props: ReadOnlyInputTextProps) -> Element {
    let ReadOnlyInputTextProps {
        source,
        placeholder,
        id,
    } = props;
    let value = source.read();

    rsx! {
        input {
            r#type: "text",
            class: "block w-full px-4 py-2 mt-2 text-gray-700 bg-white border border-gray-300 rounded-md dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600 focus:border-blue-500 dark:focus:border-blue-500 focus:outline-none focus:ring",
            readonly: true,
            id,
            placeholder: "{placeholder}",
            value: "{value}"
        }
    }
}
