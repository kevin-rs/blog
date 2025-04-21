use dioxus::prelude::*;

#[component]
pub fn Title() -> Element {
    rsx! {
        div {
            class: "text-center",
            h2 {
                id: "blog-title",
                class: "text-3xl sm:text-5xl md:text-6xl lg:text-8xl font-bold text-white tracking-tight uppercase",
                span { class: "mx-4 text-white", "Latest" }
                span { class: "text-green-500", "Insights" }
            }
            span {
                class: "block mt-4 text-white text-sm uppercase",
                "Explore our latest posts, expert tips, and updates on everything Kevin RS."
            }
        }
    }
}
