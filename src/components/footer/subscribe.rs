use dioxus::prelude::*;

#[component]
pub fn SubscribeForm() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-[25px] w-full md:w-[250px]",

            h3 {
                class: "text-green-500 font-['Lexend'] text-[30px] font-bold uppercase",
                "Subscribe"
            }

            form {
                class: "flex gap-2",

                div {
                    class: "relative w-[203px] h-[48px]",

                    input {
                        r#type: "email",
                        class: "relative w-full h-full px-4 text-text-green-600",
                        placeholder: "Enter your email",
                        aria_label: "Enter your email"
                    }
                }

                button {
                    r#type: "submit",
                    class: "w-[48px] h-[48px]",
                    "Subscribe"
                }
            }
        }
    }
}
