use crate::components::footer::footer::FooterSection;
use dioxus::prelude::*;

#[component]
pub fn LocationContact() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-[80px] w-full md:w-[210px]",

            FooterSection {
                title: "Location",
                content: "The Cosmos"
            }

            FooterSection {
                title: "Contact Us",
                content: "oss@kevin-rs.dev"
            }
        }
    }
}
