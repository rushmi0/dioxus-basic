#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::styles::slide_show_style::ANIM;

#[component]
pub fn SlideShow() -> Element {
    rsx! {
        style { {ANIM} }
        div { class: "mt-10",

            div { class: "slider",

                // CSS custom properties
                style: "--width: 600px; --height: 64px; --quantity: 10;",

                div { class: "list",

                    div { class: "item hover:-translate-y-5  card",
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

                    div { class: "item hover:-translate-y-5  card",
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

                    div { class: "item hover:-translate-y-5  card",
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

                    div { class: "item hover:-translate-y-5  card",
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

                    div { class: "item hover:-translate-y-5  card",
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

                    div { class: "item hover:-translate-y-5  card",
                        style: "--position: 6",
                        div { class: "bg-slideshow",
                            div {  class: "flex justify-center items-center",
                                a {
                                    target: "_blank",
                                    href: "https://discord.gg/XgGxMSkvUM",
                                    "👋 Community Discord"
                                }
                            }
                        }
                    }

                }

            }

        }
    }
}