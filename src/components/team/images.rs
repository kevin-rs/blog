use dioxus::prelude::*;

#[component]
pub fn Images() -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-3 gap-6 place-items-center w-full max-w-[600px] mx-auto py-8 px-4",
            aria_label: "Team member photos",

            img {
                src: asset!("/assets/team1.webp"),
                class: "w-[100px] md:w-[120px] aspect-[2/3] object-cover rounded-[80px]",
                alt: "Team member 1"
            }

            img {
                src: asset!("/assets/team2.webp"),
                class: "w-[100px] md:w-[130px] aspect-square object-cover rounded-full",
                alt: "Team member 2"
            }

            img {
                src: asset!("/assets/team3.webp"),
                class: "w-[90px] md:w-[110px] aspect-square object-cover rounded-tl-[64px] rounded-tr-[64px] rounded-bl-[64px]",
                alt: "Team member 3"
            }
        }
    }
}
