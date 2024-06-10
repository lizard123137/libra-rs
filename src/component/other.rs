use dioxus::prelude::*;
use std::collections::HashMap;

use crate::route::Route;
use crate::model::book::BookEntity;
use crate::component::book::{self, BookCard};

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav {
            class: "flex flex-col md:flex-row justify-between mb-5",
            Link {
                class: "font-bold text-white text-5xl mx-5 py-3 text-center",
                to: Route::Home {},
                h1 {
                    "Libra"
                    span {
                        class: "inline-block accent-text",
                        "RS"
                    }
                }
            }
            div {
                class: "flex flex-col md:flex-row items-center mr-5",
                Link {
                    class: "p-5 bg-clip-text hover:accent-text",
                    to: Route::BookIndex {},
                    "Browse"
                }
                Link {
                    class: "p-5 bg-clip-text hover:accent-text",
                    to: Route::About {},
                    "About us"
                }
                Link {
                    class: "p-5 bg-clip-text hover:accent-text",
                    to: Route::DebugTools {},
                    "Debug"
                }
            }
        }
        div {
            Outlet::<Route> {}
        }
    }
}

#[component]
pub fn Home() -> Element {
    let mut values = use_signal(HashMap::new);

    rsx! {
        h1 {
            class: "header-text mb-10",
            "Search for ",
            span {
                class: "accent-text",
                "books"
            }
            " at any place and any time!",
        }

        form {
            onsubmit: move |event| values.set(event.values()),
            
            div {
                class: "flex gap-x-4",
                span {
                    input { r#type: "radio", name: "search_param", value: "title", checked: true }
                    "Title"
                }
                span {
                    input { r#type: "radio", name: "search_param", value: "author" }
                    "Author"
                }
                span {
                    input { r#type: "radio", name: "search_param", value: "ISBN" }
                    "ISBN"
                }  
            }

            input {
                class: "w-full p-5 rounded-xl text-xl bg-slate-800",
                placeholder: "Type in your search regex...",
                name: "search_regex"
            }

            input {
                class: "accent-text border-2 rounded-lg m-2 p-4",
                r#type: "submit",
                "Search"
            }
        }

        pre {
            "Debug: \n {values:#?}"
        }
    }
}

#[component]
pub fn Content() -> Element {
    rsx! { 
        div {
            class: "mx-auto container overflow-hidden p-5",
            Outlet::<Route> {}        
        }
    }
}

#[component]
pub fn About() -> Element {
    rsx! {
        h1 {
            class: "header-text",
            span { class: "accent-text", "About us"}
        }
        div {
            class: "space-y-8 mx-5",
            p { "Libra-rs is a project i decided to take on in order to learn web development in Rust." }
            p { "It is essentially a database of books that you can search with either a title/author regex or using an ISBN number. You can also add your own books. In a real application the add feature would be verified for obvious reasons, however since this is just an educational project I decided not add any checks yet."}
            p {
                "It was build using the following tools:"
                ul {
                    class: "list-disc ml-5",
                    li { span { class: "accent-text", "Dioxus" }" - frontend framework" }
                    li { span { class: "accent-text", "Axum" } " - backend framework" }
                    li { span { class: "accent-text", "Sqlx" } " - async SQL toolkit"}
                    li { span { class: "accent-text", "Reqwest" } " - async http request library"}
                    li { span { class: "accent-text", "TailwindCSS" } " - CSS utility classes framework"}
                }
            }
            p { "The visual design is heavily inspired by the tailwind website. I don't have much experience with UI design, so I needed some inspiration..."}
        }
    }
}

#[component]
pub fn DebugTools() -> Element {
    rsx! {
        h1 {
            class: "header-text",
            span {
                class: "accent-text",
                "Add Book"
            }
        }

        form {
            onsubmit: move |event| async move {
                let values = event.values();

                let isbn = values.get("isbn").unwrap().as_value();
                let title = values.get("title").unwrap().as_value();
                let author = values.get("author").unwrap().as_value();
                let summary = values.get("summary").unwrap().as_value();

                let _ = book::add_book(BookEntity {
                    isbn: isbn,
                    image: None,
                    title: title,
                    author: author,
                    summary: summary,
                }).await;
            },

            class: "flex flex-col gap-y-5",
            input {
                class: "w-full p-5 rounded text-m bg-slate-800",
                placeholder: "ISBN",
                name: "isbn",
            }

            input {
                class: "w-full p-5 rounded text-m bg-slate-800",
                placeholder: "Title",
                name: "title",
            }

            input {
                class: "w-full p-5 rounded text-m bg-slate-800",
                placeholder: "Author",
                name: "author",
            }

            input {
                class: "w-full p-5 rounded text-m bg-slate-800",
                placeholder: "summary",
                name: "summary",
            }

            input {
                class: "accent-text border-2 rounded m-2 p-4",
                r#type: "submit",
                "Add to database",
            }
        }
    }
}

#[component]
pub fn Loading() -> Element {
    rsx! {
        div {
            class: "w-full flex flex-col justify-center items-center gap-y-5",
            svg {
                class: "mr-3 h-5 w-5 animate-spin",
                fill: "none",
                circle {
                    class: "opacity-25",
                    cx: "10",
                    cy: "10",
                    r: "10",
                    stroke: "white",
                    stroke_width: "4"
                }
            }
            "Loading..."
        }
    }
}

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "text-center",
            h1 { 
                class: "header-text accent-text",
                "Page not found" 
            }
            p { 
                class: "text-gray-500",
                "We are terribly sorry, but the page you requested doesn't exist."
            }
            pre {
                class: "text-gray-500",
                "log:\nattempted to navigate to: {route:?}" 
            }
        }
    }
}
