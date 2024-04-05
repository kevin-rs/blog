use next_rs::prelude::*;
use next_rs::{Image, Link};

const CONTAINER_CLASS: &str = "container mx-auto px-4";
const FLEX_CONTAINER_CLASS: &str =
    "flex items-center justify-end w-full h-screen pt-10 bg-cover bg-center relative";
const LEFT_SECTION_CLASS: &str =
    "w-full md:w-1/2 flex flex-col items-center justify-center md:items-start md:py-20";
const BANNER_TITLE_CLASS: &str = "text-5xl md:text-7xl font-bold animate-gradient pb-4";
const BANNER_SUBTITLE_CLASS: &str = "pl-2 max-w-md text-lg md:text-xl text-white font-bold";
const FLEX_CLASS: &str = "flex mx-4 my-5";
const CUSTOM_BUTTON_CLASS: &str = "bg-purple-600 text-white px-4 py-2 rounded-full text-center";
const RIGHT_SECTION_CLASS: &str = "hidden md:flex md:w-1/2 w-full";
const RELATIVE_CLASS: &str = "relative text-center mt-3 p-md-2";
const BANNER_IMAGE_CLASS: &str = "w-full h-auto md:w-auto md:h-full banner-image";

#[func]
pub fn Banner() -> Html {
    rsx! {
        <section
            id="about"
            class={FLEX_CONTAINER_CLASS}
        >
            <div
                class={CONTAINER_CLASS}
            >
                <div class={FLEX_CONTAINER_CLASS}>{ left_section() }{ right_section() }</div>
            </div>
        </section>
    }
}

fn left_section() -> Html {
    rsx! {
        <div
            class={LEFT_SECTION_CLASS}
        >
            <h1
                class={BANNER_TITLE_CLASS}
            >
                <span
                    class="flex items-center"
                >
                    <span>{ "AGI For" }</span>
                    <span
                        class="relative w-40 h-30 bg-no-repeat bg-contain"
                        style="background-image: url(images/circle.png);"
                    >
                        <span class="ml-1">{ "All" }</span>
                    </span>
                </span>
            </h1>
            <p class={BANNER_SUBTITLE_CLASS}>{ "Build Type Safe Agents Blazingly Fast!" }</p>
            <div
                class={FLEX_CLASS}
            >
                <div>
                    <Link
                        to="https://github.com/kevin-rs"
                        class={CUSTOM_BUTTON_CLASS}
                    >
                        { "Build Now" }
                    </Link>
                </div>
            </div>
        </div>
    }
}

fn right_section() -> Html {
    rsx! {
        <div
            class={RIGHT_SECTION_CLASS}
        >
            <div class={RELATIVE_CLASS}><Image class={BANNER_IMAGE_CLASS} src="images/banner.png" /></div>
        </div>
    }
}
