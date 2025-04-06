use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
pub fn LanguageToggle() -> Element {
    let I18nContext { i18n, set_language } = use_context::<I18nContext>();

    let change_language = move |lng: &str| {
        set_language.call(lng.to_string());
    };

    rsx! {
        div {
            class: "flex gap-2 items-center",

            button {
                class: format!(
                    "px-2 py-1 rounded {}",
                    if i18n().get_current_language() == "en" {
                        "bg-gray text-white"
                    } else {
                        "bg-transparent text-white border border-gray"
                    }
                ),
                onclick: move |_| change_language("en"),
                aria_label: "Switch to English",
                "EN"
            }

            button {
                class: format!(
                    "px-2 py-1 rounded {}",
                    if i18n().get_current_language() == "ar" {
                        "bg-gray text-white"
                    } else {
                        "bg-transparent text-white border border-gray"
                    }
                ),
                onclick: move |_| change_language("ar"),
                aria_label: "Switch to Arabic",
                "AR"
            }
        }
    }
}
