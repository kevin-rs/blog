use dioxus::prelude::*;

#[component]
pub fn LogoSocial() -> Element {
    let socials = vec![
        ("Facebook", "fab fa-facebook-f"),
        ("Twitter", "fab fa-x-twitter"),
        ("Instagram", "fab fa-instagram"),
        ("LinkedIn", "fab fa-linkedin-in"),
    ];

    rsx! {
        div {
            class: "flex flex-col gap-[25px] w-full md:w-[320px]",

            img {
                src: asset!("/assets/logo.png"),
                class: "w-[60px] h-[49px] object-cover",
                width: 60,
                height: 60,
                alt: "Company Logo",
            }

            div {
                class: "flex",
                {socials
                    .iter()
                    .map(|(label, icon_class)| rsx! {
                        a {
                            href: "#",
                            class: "flex items-center justify-center text-[#0d0d0d] hover:text-green-500 transition-colors text-[20px]",
                            aria_label: "{label}",
                            i {
                                class: "{icon_class} text-xl px-2"
                            }
                        }
                    })}
            }
        }
    }
}
