use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy, Default, Debug)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

#[derive(Props, PartialEq, Clone)]
pub struct ThemeProviderProps {
    pub children: Element,
}

#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    let theme = use_signal(|| Theme::default());

    use_context_provider(|| theme);

    rsx! {
        {props.children}
    }
}

#[component]
pub fn ThemeToggle() -> Element {
    let mut theme = use_context::<Signal<Theme>>();

    let toggle_theme = move |_| {
        let new_theme = if theme() == Theme::Light {
            Theme::Dark
        } else {
            Theme::Light
        };
        theme.set(new_theme);
    };

    rsx! {
        button {
            onclick: toggle_theme,
            class: "p-2 rounded-lg text-sm font-medium transition-colors duration-300 bg-gray-700 text-white hover:bg-gray-600",
            i {
                class: format!("fa {} mr-2", if theme() == Theme::Dark { "fa-moon" } else { "fa-sun" })
            }
            {if theme() == Theme::Dark { "Dark" } else { "Light" }}
        }
    }
}
