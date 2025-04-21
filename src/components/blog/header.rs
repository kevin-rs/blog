use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn BlogHeader() -> Element {
    rsx! {
        div { class: "text-white flex flex-col items-center",
            nav { class: "bg-gray-800  w-full flex justify-between items-center p-6 shadow-md",
                div { class: "text-2xl font-bold flex items-center",
                    img {
                        src: asset!("/assets/logo.webp"),
                        alt: "Logo",
                        class: "h-16 mr-2",
                        loading: "lazy",

                    }
                    span { "Kevin RS" }
                }
                Link {
                    to: Route::Home {},
                    class: "text-white bg-gray-700 rounded-lg hover:bg-white hover:text-black  px-4 py-2",
                    "Go Back →"
                }
            }
        }
    }
}
