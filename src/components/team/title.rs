use dioxus::prelude::*;

#[component]
pub fn Title() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-6 md:gap-8 items-center justify-center w-full px-6 md:px-0 h-[345px] text-center",

            h2 {
                id: "team-title",
                class: "font-['Lexend'] text-3xl sm:text-5xl md:text-6xl lg:text-8xl font-bold uppercase text-white",
                span { class: "mx-2", "The minds" }
                span { class: "text-green-500", "behind" }
            }

            div {
                class: "flex flex-col md:flex-row gap-6 md:gap-12 items-center",
                div {
                    class: "text-xl md:text-5xl font-bold uppercase text-white text-center md:text-right",
                    span { "Our" }
                    span { class: "mx-2 text-green-500", "creative" }
                    span { "team" }
                }

                a {
                    href: "https://www.linkedin.com/in/mahmoud-harmouch",
                    target: "_blank",
                    class: "px-5 md:px-8 py-2 md:py-3 inline-flex items-center gap-2 text-white bg-green-500 rounded-full font-semibold shadow-md hover:bg-green-600 transition-colors duration-200 text-sm md:text-base",
                    i { class: "fas fa-arrow-right text-white text-base md:text-lg" }
                    span { "Meet the team" }
                }
            }
        }
    }
}
