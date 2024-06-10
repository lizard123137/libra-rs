use dioxus::prelude::*;

#[component]
pub fn Error(error: ServerFnError) -> Element {
    rsx! {
        div {
            class: "text-center",
            h1 {
                class: "header-text",
                span { class: "accnet-text", "Error" }
            },
            p {
                class: "text-gray-500",
                "{error.to_string()}"
            }
        }
        
    }
}