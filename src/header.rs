use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
// use crate::theme::ThemeToggle;

struct MenuItem {
    key: &'static str,
    icon_class: &'static str,
    label: &'static str,
}

#[component]
pub fn Header() -> Element {
    let mut is_menu_open = use_signal(|| false);
    let mut is_scrolled = use_signal(|| false);

    use_effect(move || {
        let window = web_sys::window().expect("no global `window` exists");
        let mut is_scrolled = is_scrolled.clone();

        let closure = Closure::wrap(Box::new(move || {
            let window = web_sys::window().expect("no global `window` exists");
            let scroll_y = window.scroll_y().unwrap_or(0.0);
            is_scrolled.set(scroll_y > 50.0);
        }) as Box<dyn FnMut()>);

        window
            .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
            .expect("failed to add scroll event listener");
    });

    let toggle_menu = move |_| {
        is_menu_open.set(!is_menu_open());
    };

    let menu_items = vec![
        MenuItem {
            key: "home",
            icon_class: "fa-solid fa-house-chimney",
            label: "Home",
        },
        MenuItem {
            key: "features",
            icon_class: "fa-solid fa-cubes",
            label: "Features",
        },
        MenuItem {
            key: "testimonials",
            icon_class: "fa-solid fa-comments",
            label: "Testimonials",
        },
        MenuItem {
            key: "team",
            icon_class: "fa-solid fa-people-group",
            label: "Team",
        },
        MenuItem {
            key: "blog",
            icon_class: "fa-solid fa-newspaper",
            label: "Blog",
        },
    ];

    let header_class = format!(
        "fixed top-0 left-0 right-0 z-50 transition-all duration-300 {}",
        if is_scrolled() {
            "bg-black backdrop-blur-sm"
        } else {
            ""
        }
    );

    let menu_icon_class = format!(
        "fa {}",
        if is_menu_open() {
            "fa-times"
        } else {
            "fa-bars"
        }
    );

    rsx! {
        header {
            class: "{header_class}",

            div {
                class: "bg-black flex justify-between items-center w-full max-w-[1260px] mx-auto px-4 relative z-[2]",

                img {
                    src: asset!("/assets/logo.png"),
                    alt: "Company Logo",
                    class: "w-12 h-12 shrink-0"
                }

                nav {
                    class: "hidden md:flex absolute md:relative left-1/2 md:left-0 transform -translate-x-1/2 md:transform-none w-auto gap-[50px] justify-center items-center",
                    aria_label: "Main Navigation",
                    ul {
                        class: "flex gap-[50px]",
                        for item in &menu_items {
                            li {
                                class: "mx-4",
                                a {
                                    href: format!("#{}", item.key),
                                    class: "h-[23px] font-['Lexend'] text-[18px] font-normal leading-[22.5px] text-white uppercase whitespace-nowrap hover:text-green-500 transition-colors",
                                    i { class: format!("{} mr-2", item.icon_class), aria_hidden: "true" }
                                    {item.label}
                                }
                            }
                        }
                    }
                }

                div {
                    class: "flex items-center gap-4",
                    button {
                        class: "md:hidden text-white p-2",
                        onclick: toggle_menu,
                        aria_expanded: "{is_menu_open()}",
                        aria_label: "Toggle menu",

                        i { class: "{menu_icon_class}" }
                    }

                    div {
                        class: "hidden md:flex gap-4 items-center",
                        // ThemeToggle {}
                    }
                }
            }

            if is_menu_open() {
                nav {
                    class: "md:hidden bg-black w-full py-4",
                    aria_label: "Mobile Navigation",
                    ul {
                        class: "flex flex-col gap-4 px-4",
                        for item in &menu_items {
                            li {
                                a {
                                    href: format!("#{}", item.key),
                                    class: "flex items-center h-[23px] font-['Lexend'] text-[18px] font-normal leading-[22.5px] text-white uppercase whitespace-nowrap hover:text-green-500 transition-colors",
                                    onclick: move |_| is_menu_open.set(false),
                                    i { class: format!("{} mr-2", item.icon_class), aria_hidden: "true" }
                                    {item.label}
                                }
                            }
                        }
                        li {
                            class: "mt-4 flex gap-4 items-center",
                            // ThemeToggle {}
                        }
                    }
                }
            }
        }
    }
}
