pub(crate) mod grid;
pub(crate) mod item;

use crate::components::features::grid::Grid;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
struct Feature {
    icon: Element,
    title: &'static str,
    description: &'static str,
}

#[component]
pub fn Features() -> Element {
    let features = vec![
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "text-4xl fa-solid fa-infinity",
            }},
            title: "Large Mathematical Models (LMMs)",
            description: "Go beyond text. Empower your agents to encode and simulate reality precisely using pure mathematical equations.",
        },
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "text-4xl fa-solid fa-bolt",
            }},
            title: "Blazing Fast Native Execution",
            description: "Built entirely in pure Rust, offering zero-cost abstractions and fearless concurrency for real-time ASI capabilities.",
        },
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "text-4xl fa-solid fa-brain",
            }},
            title: "Equation Discovery & Simulation",
            description: "Agents capable of autonomous symbolic regression and simulation to predict outcomes before they happen.",
        },
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "text-4xl fa-solid fa-network-wired",
            }},
            title: "Multi-Agent Architecture",
            description: "Deploy swarms of fault-tolerant agents that seamlessly collaborate to solve complex, multi-step problems.",
        },
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "text-4xl fa-solid fa-eye",
            }},
            title: "Inspired by the Eye of Horus",
            description: "A philosophy of deep perception and wholeness. Let your AI see the underlying mathematical structure of the universe.",
        },
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "text-4xl fa-solid fa-shield-halved",
            }},
            title: "Safe and Type-Driven",
            description: "Leverage Rust's uncompromising safety guarantees to build autonomous agents you can trust in production.",
        },
    ];

    rsx! {
        section { id: "features", class: "bg-gray-100 py-28 px-16 md:px-4 font-roboto flex min-h-screen justify-center",
            div { class: "",
                Grid { features: features }
            }
        }
    }
}
