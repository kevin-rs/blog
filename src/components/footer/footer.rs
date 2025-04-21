use dioxus::prelude::*;

#[component]
pub fn FooterSection(title: String, content: String) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-[25px]",

            h3 {
                class: "text-green-500 font-['Lexend'] text-[30px] font-bold uppercase",
                "{title}"
            }

            p {
                class: "text-white font-['Lexend'] text-[15px]",
                "{content}"
            }
        }
    }
}
