#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::styles::slide_show_style::CSS;

#[component]
pub fn SlideShow() -> Element {
    rsx! {
        style { {CSS} }
        div { class: "mt-10",

            div { class: "slider",

                // CSS custom properties
                style: "--width: 600px; --height: 64px; --quantity: 10;",

                div { class: "list",

                    div { class: "item card hover:-translate-y-2",
                        style: "--position: 1",
                        div { class: "bg-slideshow",
                            div {  class: "flex justify-center items-center",
                                a {
                                    target: "_blank",
                                    href: "https://dioxuslabs.com/learn/0.5/",
                                    "📚 Learn Dioxins"
                                }
                            }
                        }
                    }

                    div { class: "item card hover:-translate-y-2",
                        style: "--position: 2",
                        div { class: "bg-slideshow",
                            div {  class: "flex justify-center items-center",
                                a {
                                    target: "_blank",
                                    href: "https://dioxuslabs.com/awesome",
                                    "🚀 Awesome Dioxus"
                                }
                            }
                        }
                    }

                    div { class: "item card hover:-translate-y-2",
                        style: "--position: 3",
                        div { class: "bg-slideshow",
                            div {  class: "flex justify-center items-center",
                                a {
                                    target: "_blank",
                                    href: "https://github.com/dioxus-community/",
                                    "📡 Community Libraries"
                                }
                            }
                        }
                    }

                    div { class: "item card hover:-translate-y-2",
                        style: "--position: 4",
                        div { class: "bg-slideshow",
                            div {  class: "flex justify-center items-center",
                                a {
                                    target: "_blank",
                                    href: "https://github.com/DioxusLabs/dioxus-std",
                                    "⚙️ Dioxus Standard Library"
                                }
                            }
                        }
                    }

                    div { class: "item card hover:-translate-y-2",
                        style: "--position: 5",
                        div { class: "bg-slideshow",
                            div {  class: "flex justify-center items-center",
                                a {
                                    target: "_blank",
                                    href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                                    "💫 VSCode Extension"
                                }
                            }
                        }
                    }

                }
            }
        }
    }
}