use crate::components::features::item::FeatureItem;
use crate::components::features::Feature;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FeatureGridProps {
    features: Vec<Feature>,
}

#[component]
pub fn Grid(props: FeatureGridProps) -> Element {
    rsx! {
        div { class: "mt-8 grid grid-cols-1 md:grid-cols-3 gap-12",

            for feature in &props.features {
                div {
                    class: "border border-black shadow-md p-6 rounded-lg bg-gray-900 cursor-pointer hover:bg-gray-800 hover:shadow-lg transition-all duration-300",

                    FeatureItem {
                        icon: feature.icon.clone(),
                        title: feature.title.clone(),
                        description: feature.description.clone(),
                    }
                }
            }
        }
    }
}
