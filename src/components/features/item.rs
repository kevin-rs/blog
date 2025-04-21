use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ItemProps {
    icon: Element,
    title: String,
    description: String,
}

#[component]
pub fn FeatureItem(props: ItemProps) -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            div { class: "w-12 h-12", {props.icon} }
            h3 { class: "text-2xl font-bold leading-snug", "{props.title}" }
            p { class: "text-lg leading-relaxed", "{props.description}" }
        }
    }
}
