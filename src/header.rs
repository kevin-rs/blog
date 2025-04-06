use crate::language::LanguageToggle;
use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;
use wasm_bindgen::prelude::*;
// use crate::theme::ThemeToggle;

struct MenuItem {
    key: &'static str,
    icon_class: &'static str,
}

#[component]
pub fn Header() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    let mut is_menu_open = use_signal(|| false);
    let mut is_scrolled = use_signal(|| false);

    use_effect(move || {
        let scroll_callback = Closure::wrap(Box::new(move || {
            let scroll_y = web_sys::window().unwrap().scroll_y().unwrap_or(0.0);
            is_scrolled.set(scroll_y > 50.0);
        }) as Box<dyn FnMut()>);

        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("scroll", scroll_callback.as_ref().unchecked_ref())
            .unwrap();

        scroll_callback.forget();
    });

    let toggle_menu = move |_| {
        is_menu_open.set(!is_menu_open());
    };

    let menu_items = vec![
        MenuItem {
            key: "home",
            icon_class: "fa fa-home",
        },
        MenuItem {
            key: "services",
            icon_class: "fa fa-tools",
        },
        MenuItem {
            key: "features",
            icon_class: "fa fa-star",
        },
        MenuItem {
            key: "agents",
            icon_class: "fa fa-folder",
        },
        MenuItem {
            key: "blog",
            icon_class: "fa fa-blog",
        },
        MenuItem {
            key: "contact",
            icon_class: "fa fa-phone",
        },
    ];
    let header_class = format!(
        "fixed top-0 left-0 right-0 z-50 transition-all duration-300 {}",
        if is_scrolled() {
            "bg-black backdrop-blur-sm py-2"
        } else {
            "py-4"
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
                                    class: "h-[23px] font-['Lexend'] text-[18px] font-normal leading-[22.5px] text-white uppercase whitespace-nowrap hover:text-[#e26e23] transition-colors",
                                    i { class: format!("{} mr-2", item.icon_class), aria_hidden: "true" }
                                    {i18n().t(&format!("header.menu.{}", item.key))}
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
                        aria_label: "{i18n().t(\"accessibility.menuToggle\")}",

                        i { class: "{menu_icon_class}" }
                    }

                    div {
                        class: "hidden md:flex gap-4 items-center",
                        LanguageToggle {}
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
                                    class: "flex items-center h-[23px] font-['Lexend'] text-[18px] font-normal leading-[22.5px] text-white uppercase whitespace-nowrap hover:text-[#e26e23] transition-colors",
                                    onclick: move |_| is_menu_open.set(false),
                                    i { class: format!("{} mr-2", item.icon_class), aria_hidden: "true" }
                                    {i18n().t(&format!("header.menu.{}", item.key))}
                                }
                            }
                        }
                        li {
                            class: "mt-4 flex gap-4 items-center",
                            LanguageToggle {}
                            // ThemeToggle {}
                        }
                    }
                }
            }
        }
    }
}
