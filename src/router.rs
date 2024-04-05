use crate::pages::landing::LandingPage;
use next_rs::prelude::*;

pub fn switch(route: String) -> Html {
    match route.as_str() {
        "/" => rsx! {<LandingPage />},
        _ => rsx! {<></>},
    }
}
