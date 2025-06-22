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
        "fa-gauge-high",
        "fa-network-wired",
        "fa-cubes",
        "fa-gears",
        "fa-robot",
        "fa-code-branch",
    ];

    let features_data = vec![
        (
            "Blazing-Fast Native Execution",
            "Leverage Rust's unmatched speed and safety guarantees to build high-performance agentic workflows without the overhead of traditional runtimes.",
        ),
        (
            "Agent-Oriented Architecture",
            "Model and deploy autonomous, reasoning-capable agents with built-in task orchestration and memory systems tailored for advanced AI behavior.",
        ),
        (
            "Composable, Modular Systems",
            "Build workflows from composable primitives that snap together naturally, making it easy to scale and adapt agent logic over time.",
        ),
        (
            "Tooling Built for Rust Devs",
            "First-class developer experience with ergonomic APIs, native async support, and deep integration into the Rust ecosystem.",
        ),
        (
            "Intelligent Workflow Automation",
            "Design systems that observe, reason, and act, enabling agents to autonomously complete multi-step tasks and adapt to dynamic goals.",
        ),
        (
            "Built for Full Autonomy",
            "From task planning to execution, give your agents end-to-end control with minimal boilerplate and maximum flexibility.",
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
