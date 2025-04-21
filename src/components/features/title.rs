use dioxus::prelude::*;

#[component]
pub fn Title() -> Element {
    rsx! {
        div {
            class: "text-center",
            h2 {
                id: "features-title",
                class: "text-3xl sm:text-5xl md:text-6xl lg:text-8xl font-bold text-white tracking-tight uppercase",
                span { class: "mx-4 text-white", "Discover" }
                span { class: "text-green-500", "The Future" }
                br {}
                span { class: "mx-4 text-white", "of" }
                span { class: "text-green-500", "AGI" }
                span { class: "mx-4 text-white", "Research" }
            }
            span {
                class: "block mt-4 text-white text-sm uppercase",
                "Cutting-edge breakthroughs, one innovation at a time."
            }
        }
    }
}
