#![allow(non_snake_case)]

use dioxus::prelude::*;

const IMG_BANNER: &str = manganis::mg!(file("src/assets/multiplatform-dark.svg"));

#[component]
pub fn Banner() -> Element {
    rsx! {
        div { class: "container mx-auto",

            div { class: "flex justify-between items-center pt-[5%] col-lg-12 col-md-12 col-sm-12 flex-col-reverse lg:flex-row-reverse xl:flex-row grid lg:grid-rows-none lg:grid-cols-4 grid-rows-4",

                div { class: "lg:p-[50px] text-center lg:text-start leading-none md:col-span-2 lg:row-span-2",

                    h1 { class: "lg:text-[96px] font-bold text-[#935CD1] text-[48px] sm:text-[48px]", "Dioxus v0.5" }
                    h2 { class: "lg:text-[96px] font-bold text-[48px] sm:text-[48px]", "Webassembly" }

                    p { class: "mt-10 lg:text-[24px] text-[21px] sm:text-[21px] text-balance leading-normal",
                        "Le Lorem Ipsum est simplement du faux texte employé dans la composition et la mise en page avant impression. Le Lorem Ipsum est le faux texte standard de"
                    }

                }

                div { class: "md:col-span-2 md:row-span-4 lg:row-span-2 row-span-4 order-first lg:order-last",
                    div { class: "lg:relative lg:w-[671px] lg:h-[671px]",
                        img { class: "inset-0 object-cover",
                            src: "{IMG_BANNER}"
                        }
                    }
                }

            }
        }
    }
}