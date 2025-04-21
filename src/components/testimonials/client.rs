use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ClientProps {
    name: String,
    role: String,
    testimonial: Option<String>,
    image_url: Asset,
    overlay_url: Option<Asset>,
    is_active: bool,
}

#[component]
pub fn Client(props: ClientProps) -> Element {
    let outer_class = "relative w-[491px] h-[393px] shrink-0 flex items-center justify-center";

    let content_class = if props.is_active {
        "relative w-full h-full scale-100 z-20 transition-transform duration-300 rounded-[32px] overflow-hidden bg-text-green-500 shadow-xl"
    } else {
        "relative w-full h-full scale-80 z-10 transition-transform duration-300 rounded-[32px] overflow-hidden bg-text-green-500 shadow-md"
    };

    rsx! {
        div { class: outer_class,
            div { class: content_class,
                div { class: "justify-center relative",
                    img {
                        src: props.image_url,
                        class: "w-full h-full object-cover",
                        alt: ""
                    }
                    if let Some(overlay) = props.overlay_url {
                        img {
                            src: overlay,
                            class: "absolute top-8 left-8 w-1/2 object-cover pointer-events-none",
                            alt: "Overlay"
                        }
                    }
                }

                if props.is_active {
                    div { class: "absolute left-6 bottom-0 w-[204px] p-4 text-white z-10",
                    h3 { class: "text-2xl font-bold uppercase", "{props.name}" }
                    p { class: "text-sm capitalize", "{props.role}" }
                    if let Some(testimonial) = props.testimonial {
                        p { class: "mt-4 text-sm leading-6", "{testimonial}" }
                    }
                    div { class: "flex gap-2 mt-3",
                    div { class: "flex gap-2 mt-3 text-green-400",
                        for _ in 0..5 {
                                i { class: "fas fa-star" }
                            }
                        }
                    }
                }
                } else {
                    div { class: "absolute left-6 bottom-10 w-[204px] p-4 text-white z-10",
                    h3 { class: "text-2xl font-bold uppercase", "{props.name}" }
                    p { class: "text-sm capitalize", "{props.role}" }
                    if let Some(testimonial) = props.testimonial {
                        p { class: "mt-4 text-sm leading-6", "{testimonial}" }
                    }
                }
                }
            }
        }
    }
}
