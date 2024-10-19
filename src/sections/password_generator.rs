use dioxus::prelude::*;
use dioxus_logger::tracing::*;
use rand::Rng;

use crate::components::form::input_text::InputText;
use crate::components::form::range::Range;
use crate::components::form::read_only_input_text::ReadOnlyInputText;
use crate::components::form::select::Select;

use rand::distributions::{Alphanumeric, DistString};

#[derive(Debug, Clone, Copy, PartialEq, enum_iterator::Sequence)]
enum Dictionary {
    Readable,
    Alphanumeric,
    Ascii,
    SpanishKeyboard,
    Unicode,
    Custom,
}

impl From<&str> for Dictionary {
    fn from(value: &str) -> Self {
        match value {
            "Readable" => Dictionary::Readable,
            "Alphanumeric" => Dictionary::Alphanumeric,
            "ASCII" => Dictionary::Ascii,
            "Spanish Keyboard" => Dictionary::SpanishKeyboard,
            "Unicode" => Dictionary::Unicode,
            "Custom" => Dictionary::Custom,
            _ => Dictionary::Ascii,
        }
    }
}

impl From<Dictionary> for String {
    fn from(value: Dictionary) -> Self {
        match value {
            Dictionary::Readable => "Readable".to_string(),
            Dictionary::Alphanumeric => "Alphanumeric".to_string(),
            Dictionary::Ascii => "ASCII".to_string(),
            Dictionary::SpanishKeyboard => "Spanish Keyboard".to_string(),
            Dictionary::Unicode => "Unicode".to_string(),
            Dictionary::Custom => "Custom".to_string(),
        }
    }
}

fn get_unicode_password(number_of_characters: i64) -> String {
    let my_str = rand_utf8::rand_utf8(
        &mut rand::thread_rng(),
        number_of_characters.try_into().unwrap(),
    );
    my_str.to_string()
}

fn get_alphanumeric_password(number_of_characters: i64) -> String {
    Alphanumeric.sample_string(
        &mut rand::thread_rng(),
        number_of_characters.try_into().unwrap(),
    )
}

fn get_custom_password(custom_dictionary: &str, number_of_characters: i64) -> String {
    if custom_dictionary.is_empty() {
        return "".to_string();
    }
    let custom_dictionary = custom_dictionary.chars().collect::<Vec<char>>();
    let mut rng = rand::thread_rng();
    let one_char = || custom_dictionary[rng.gen_range(0..custom_dictionary.len())];
    std::iter::repeat_with(one_char)
        .take(number_of_characters.try_into().unwrap())
        .collect()
}

const ASCII_DICTIONARY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
const READABLE_DICTIONARY: &str = "abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ23456789";
const SPANISH_DICTIONARY: &str = "qwertyuiopasdfghjklñçzxcvbnm1234567890QWERTYUIOPASDFGHJKLÑÇZXCVBNM,.;:-_^*+[]{}!|\\\"@·#$%&/()=?¿?'¡àèìòùÀÈÌÒÙÁÉÍÓÚáéíóú<>";

fn get_password(
    dictionary: Dictionary,
    custom_dictionary: &str,
    number_of_characters: i64,
) -> String {
    match dictionary {
        Dictionary::Alphanumeric => get_alphanumeric_password(number_of_characters),
        Dictionary::Ascii => get_custom_password(ASCII_DICTIONARY, number_of_characters),
        Dictionary::Readable => get_custom_password(READABLE_DICTIONARY, number_of_characters),
        Dictionary::SpanishKeyboard => {
            get_custom_password(SPANISH_DICTIONARY, number_of_characters)
        }
        Dictionary::Unicode => get_unicode_password(number_of_characters),
        Dictionary::Custom => get_custom_password(custom_dictionary, number_of_characters),
    }
}

#[component]
pub fn PasswordGenerator() -> Element {
    info!("Creating Password Generator");

    const DEFAULT_DICTIONARY: Dictionary = Dictionary::Ascii;
    const DEFAULT_SIZE: i64 = 32;
    const MAX_SIZE: i64 = 64;

    let signal_dictionary = use_signal(|| Into::<String>::into(DEFAULT_DICTIONARY));
    let signal_custom_dictionary = use_signal(|| "".to_string());
    let signal_number_of_characters = use_signal(|| DEFAULT_SIZE);
    let mut signal_password = use_signal(|| "".to_string());

    let dictionary = Dictionary::from(signal_dictionary.read().as_str());
    info!("Dictionary: {:?}", dictionary);

    signal_password.set(get_password(
        dictionary,
        signal_custom_dictionary.read().as_str(),
        *signal_number_of_characters.read(),
    ));

    let selectDictionaryPassword = rsx! {
        Select {
            source: signal_dictionary,
            default: Into::<String>::into(DEFAULT_DICTIONARY),
            values: enum_iterator::all::<Dictionary>().map(|x| x.into()).collect(),
            id: "select-dictionary"
        }
    };
    rsx! {
        div { class: "relative bg-cover bg-center bg-no-repeat py-8 md:py-4 lg:py-8",
            div { class: "absolute inset-0 z-20 bg-gradient-to-r from-hero-gradient-from to-hero-gradient-to bg-cover bg-center bg-no-repeat" }
            div { class: "container relative z-30 pt-2 sm:pt-5 md:pt-0 lg:pt-8",
                div { class: "flex flex-col items-center justify-center lg:flex-row",
                    div { class: "pt-8 sm:pt-10 lg:pl-8 md:pt-0 lg:pt-0",
                        h1 { class: "text-center font-header text-2xl text-white-text sm:text-left sm:text-3xl md:text-3xl",
                            "WebAssembly Local Password Generator"
                        }
                    }
                }
            }
        }
        section { class: "max-w-4xl p-6 mx-auto bg-indigo-600 rounded-md shadow-none lg:shadow-md lg:mt-8",
            form {
                label {
                    r#for: "password-result",
                    class: "text-primary dark:text-gray-200",
                    "Password result"
                }
                div {
                    ReadOnlyInputText { source: signal_password, placeholder: "password", id: "password-result" }
                }
                if dictionary != Dictionary::Custom {
                    div { class: "grid grid-cols-1 gap-6 mt-4 sm:grid-cols-1",
                        div {
                            label {
                                r#for: "select-dictionary",
                                class: "text-primary dark:text-gray-200",
                                "Select character set"
                            }
                            {selectDictionaryPassword}
                        }
                    }
                } else {
                    div { class: "grid grid-cols-1 gap-6 mt-4 sm:grid-cols-2",
                        div {
                            label {
                                r#for: "select-dictionary",
                                class: "text-primary dark:text-gray-200",
                                "Select character set"
                            }
                            {selectDictionaryPassword}
                        }
                        div {
                            label {
                                r#for: "custom-dictionary",
                                class: "text-primary dark:text-gray-200",
                                "Custom Dictionary Characters"
                            }
                            InputText {
                                source: signal_custom_dictionary,
                                placeholder: "eg. abc123456",
                                id: "custom-dictionary"
                            }
                        }
                    }
                }
                div { class: "grid grid-cols-1 gap-6 mt-4",
                    div {
                        label {
                            r#for: "range-password",
                            class: "text-primary dark:text-gray-200",
                            "Number of characters"
                        }
                        Range {
                            source: signal_number_of_characters,
                            min: 1,
                            max: MAX_SIZE,
                            step: 1,
                            id: "range-password"
                        }
                    }
                }
                div { class: "grid grid-cols-2 gap-6 mt-4",
                    div {
                        button {
                            r#type: "button",
                            onclick: move |_| {
                                signal_password
                                    .set(
                                        get_password(
                                            dictionary,
                                            signal_custom_dictionary.read().as_str(),
                                            *signal_number_of_characters.read(),
                                        ),
                                    );
                            },
                            prevent_default: true,
                            class: "px-6 py-2 w-full leading-5 text-white-text transition-colors duration-200 transform bg-tertiary rounded-md hover:bg-quaternary focus:outline-none focus:bg-gray-600",
                            "Regenerate"
                        }
                    }
                    div {
                        button {
                            r#type: "button",
                            onclick: move |_| {
                                let password = signal_password.read();
                                crate::utils::clipboard::set_clipboard(password.as_str());
                            },
                            prevent_default: true,
                            class: "px-6 py-2 w-full leading-5 text-white-text transition-colors duration-200 transform bg-tertiary rounded-md hover:bg-quaternary focus:outline-none focus:bg-gray-600",
                            "Copy"
                        }
                    }
                }
            }
        }
    }
}

/*
*/
