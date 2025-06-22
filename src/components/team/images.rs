use dioxus::prelude::*;

#[component]
pub fn Images() -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-3 gap-6 place-items-center w-full py-8 px-4",
            aria_label: "Team member photos",

            img {
                src: asset!("/assets/team1.webp"),
                class: "w-[290px] md:w-[310px] object-cover rounded-tl-[34px] rounded-bl-[34px]",
                alt: "Team member 1"
            }

            img {
                src: asset!("/assets/team2.webp"),
                class: "w-[320px] md:w-[350px] object-cover rounded-full",
                alt: "Team member 2"
            }

            img {
                src: asset!("/assets/team3.webp"),
                class: "w-[290px] md:w-[310px] object-cover rounded-tr-[34px] rounded-bl-[34px]",
                alt: "Team member 3"
            }
        }
    }
}
