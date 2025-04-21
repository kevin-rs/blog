pub mod images;
pub mod title;

use crate::components::team::images::Images;
use crate::components::team::title::Title;

use dioxus::prelude::*;

#[component]
pub fn Team() -> Element {
    rsx! {
        section {
            id: "team",
            class: "min-h-screen mb-[-6px] py-12 md:py-24",
            aria_labelledby: "team-title",

            Title {}

            div {
                class: "mt-16",
                Images {}
            }
        }
    }
}
