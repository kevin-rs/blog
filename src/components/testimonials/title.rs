use dioxus::prelude::*;

#[component]
pub fn Title() -> Element {
    rsx! {
        div {
            class: "text-center",

            h2 {
                id: "testimonials-title",
                class: "text-3xl sm:text-5xl md:text-6xl lg:text-8xl font-bold text-white tracking-tight uppercase",

                span { class: "mx-4 text-white", "What" }
                span { class: "text-green-500", "Crabs" }
                br {}
                span { class: "mx-4 text-white", "say" }
                span { class: "text-green-500", "about" }
                span { class: "mx-4 text-white", "us 🦀" }
                span { class: "text-white", "?" }
            }

            span {
                class: "block mt-4 text-white text-sm uppercase",
                "Unfiltered thoughts from Rustaceans who definitely didn't unwrap() this by accident"
            }
        }
    }
}
