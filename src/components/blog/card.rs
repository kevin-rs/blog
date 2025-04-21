use crate::blog::router_blog::BookRoute as BlogRoute;
use crate::router::Route;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq, Debug)]
pub struct BlogHomeCardProps {
    pub title: String,
    pub route: BlogRoute,
    pub desc: String,
    pub img: Option<String>,
    pub created_at: String,
    pub category: String,
    pub slug: String,
}

#[component]
pub fn BlogHomeCard(props: BlogHomeCardProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col border border-gray-300 rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition transform hover:scale-105",

            if let Some(img_url) = &props.img {
                img {
                    src: "{img_url}",
                    alt: "{props.title}",
                    class: "w-full h-48 object-cover",
                    loading: "lazy",

                }
            }

            div {
                class: "p-4 flex flex-col gap-2 bg-white",

                div {
                    class: "text-xs font-semibold text-gray-600 uppercase",
                    "{props.category}"
                }

                h2 {
                    class: "text-lg font-bold text-gray-800",
                    "{props.title}"
                }

                div {
                    class: "justify-between flex",
                    span {
                        class: "text-gray-400",
                        "{props.desc.chars().take(30).collect::<String>()}...",
                    }

                    Link {
                        class: "text-indigo-500 inline-flex items-center",
                        to: Route::BlogPost { child: props.route },
                        "Read more"
                        ArrowRight {}
                    }
                }

                div {
                    class: "text-gray-500 text-xs mt-2",
                    "{props.created_at}"
                }
            }
        }
    }
}

pub(crate) fn ArrowRight() -> Element {
    rsx! {
        svg {
            class: "w-4 h-4 ml-1",
            stroke_linejoin: "round",
            stroke: "currentColor",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "2",
            stroke_linecap: "round",
            path { d: "M5 12h14M12 5l7 7-7 7" }
        }
    }
}
