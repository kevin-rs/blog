pub(crate) mod grid;
pub(crate) mod item;
pub(crate) mod title;

use crate::components::features::grid::Grid;
use crate::components::features::title::Title;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Feature {
    icon: Element,
    title: String,
    description: String,
}

#[component]
pub fn Features() -> Element {
    let icons = vec![
        "fa-infinity",
        "fa-bolt",
        "fa-brain",
        "fa-network-wired",
        "fa-eye",
        "fa-shield-halved",
    ];

    let features_data = vec![
        (
            "Large Mathematical Models (LMMs)",
            "Go beyond text. Empower your super agents to encode and simulate reality precisely using pure mathematical equations.",
        ),
        (
            "Blazing Fast Native Execution",
            "Built entirely in pure Rust, offering zero-cost abstractions and fearless concurrency for real-time ASI capabilities.",
        ),
        (
            "Equation Discovery & Simulation",
            "Super agents capable of autonomous symbolic regression and simulation to predict outcomes before they happen.",
        ),
        (
            "Multi-Agent Architecture",
            "Deploy swarms of fault-tolerant super agents that seamlessly collaborate to solve complex, multi-step problems.",
        ),
        (
            "Inspired by the Eye of Horus",
            "A philosophy of deep perception and wholeness. Let your AI see the underlying mathematical structure of the universe.",
        ),
        (
            "Safe and Type-Driven",
            "Leverage Rust's uncompromising safety guarantees to build autonomous super agents you can trust in production.",
        ),
    ];

    let features = features_data
        .into_iter()
        .enumerate()
        .map(|(index, (title, description))| Feature {
            icon: rsx!(i {
                width: 30,
                height: 30,
                class: format!(
                    "text-4xl fa-solid {}",
                    icons.get(index).unwrap_or(&"fa-circle-question")
                ),
            }),
            title: title.to_string(),
            description: description.to_string(),
        })
        .collect::<Vec<Feature>>();

    rsx! {
        section { id: "features", class: "py-28 px-16 md:px-4 font-roboto flex min-h-screen justify-center",
            div { class: "",
                Title {}
                Grid { features: features }
            }
        }
    }
}
