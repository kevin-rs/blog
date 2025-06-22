use crate::components::blog::Blog;
use crate::components::features::Features;
use crate::components::footer::Footer;
use crate::components::team::Team;
use crate::components::testimonials::Testimonials;
use crate::header::Header;
use crate::hero::Hero;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "main-container bg-black h-screen",
            Header {}
            main {
                id: "main-content",
                Hero {}
                Features {}
                Testimonials {}
                Team {}
                Blog {}
            }
            Footer {}
        }
    }
}
