use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct RangeProps {
    pub source: Signal<i64>,
    pub min: i64,
    pub max: i64,
    #[props(default = 1)]
    pub step: i64,
    #[props(into, default)]
    pub id: Option<String>,
}

#[component]
pub fn Range(props: RangeProps) -> Element {
    let RangeProps {
        mut source,
        min,
        max,
        step,
        id,
    } = props;
    let value = source.read();

    rsx! {
        div { class: "flex flex-row items-center justify-between",
            input {
                oninput: move |event| source.set(event.value().parse::<i64>().unwrap_or(min)),
                onchange: move |event| source.set(event.value().parse::<i64>().unwrap_or(min)),
                r#type: "range",
                min,
                max,
                step,
                class: "block w-full py-2 mt-2 text-gray-700 bg-white border border-gray-300 rounded-md dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600 focus:border-blue-500 dark:focus:border-blue-500 focus:outline-none focus:ring",
                id: id.clone(),
                value: "{value}"
            }
            label { class: "py-2 mt-2 ml-2 text-primary dark:text-gray-200", "{value}" }
        }
    }
}
