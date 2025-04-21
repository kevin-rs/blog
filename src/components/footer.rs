pub mod contacts;
pub mod footer;
pub mod services;
pub mod social;
pub mod subscribe;

use crate::components::footer::contacts::LocationContact;
use crate::components::footer::footer::FooterSection;
use crate::components::footer::services::ServicesList;
use crate::components::footer::social::LogoSocial;
use crate::components::footer::subscribe::SubscribeForm;

use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    let services = vec![
        "AI Consulting".to_string(),
        "ML Training".to_string(),
        "Model Deployment".to_string(),
        "Edge AI Solutions".to_string(),
    ];

    rsx! {
        footer {
            class: "w-full py-16 bg-[#0d0d0d]",
            aria_labelledby: "footer-heading",

            h2 {
                id: "footer-heading",
                class: "sr-only",
                "Footer"
            }

            div {
                class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-10 max-w-[1313.667px] mx-auto px-4 md:px-0 relative z-[220]",

                LogoSocial {}

                LocationContact {}

                ServicesList { services }

                SubscribeForm {}
            }
        }
    }
}
