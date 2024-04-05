use next_rs::prelude::*;
use next_rs::router::*;

use crate::router::switch;

#[func]
pub fn App() -> Html {
    rsx! { <NextRouter><Switch render={switch} /></NextRouter> }
}
