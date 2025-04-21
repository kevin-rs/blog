use dioxus::prelude::*;

#[component]
pub fn ServicesList(services: Vec<String>) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-[25px] w-full md:w-[233px]",

            h3 {
                class: "text-green-500 font-['Lexend'] text-[30px] font-bold uppercase",
                "Services"
            }

            ul {
                class: "flex flex-col gap-[10px]",
                for (i, item) in services.iter().enumerate() {
                    li {
                        key: "{i}",
                        class: "text-white font-['Lexend'] text-[15px]",
                        a {
                            href: "#",
                            class: "hover:text-green-500 transition-colors",
                            "{item}"
                        }
                    }
                }
            }
        }
    }
}
