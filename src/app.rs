#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::pages::HomePage;

const STYLE: &str = manganis::mg!(file("src/main.css"));
const ICON: &str = manganis::mg!(file("src/assets/favicon.ico"));

const _FONT: &str = manganis::mg!(font().families(["JetBrains Mono"]));
const _TAILWIND: &str = manganis::mg!(file("public/tailwind.css"));

#[component]
pub fn App() -> Element {
    rsx! {
        link { rel: "icon", href: ICON }
        link { rel: "stylesheet", href: STYLE }
        HomePage {}
    }
}
