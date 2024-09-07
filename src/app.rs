#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::pages::HomePage;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));
const STYLE: &str = manganis::mg!(file("src/styles/main.css"));


#[component]
pub fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: STYLE }
        HomePage {}
    }
}
