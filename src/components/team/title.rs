use dioxus::prelude::*;

#[component]
pub fn Title() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-6 md:gap-8 items-center justify-center w-full px-6 md:px-0 md:w-[907px] h-[345px] text-center",

            h2 {
                id: "team-title",
                class: "font-['Lexend'] text-2xl md:text-8xl md:text-[150px] font-bold leading-tight tracking-tight uppercase text-white z-[170]",
                span { class: "mx-2", "The minds" }
                span { class: "text-green-500", "behind" }
            }

            div {
                class: "flex flex-col md:flex-row gap-6 md:gap-12 items-center z-[171]",
                div {
                    class: "text-xl md:text-5xl font-bold leading-tight uppercase text-white text-center md:text-right",
                    span { "Our" }
                    span { class: "mx-2 text-green-500", "creative" }
                    span { "team" }
                }

                a {
                    href: "https://www.linkedin.com/in/mahmoud-harmouch",
                    target: "_blank",
                    class: "z-[173] px-5 md:px-8 py-2 md:py-3 inline-flex items-center gap-2 text-white bg-green-500 rounded-full font-semibold shadow-md hover:bg-green-600 transition-colors duration-200 text-sm md:text-base",
                    i { class: "fas fa-arrow-right text-white text-base md:text-lg" }
                    span { "Meet the team" }
                }
            }
        }
    }
}
