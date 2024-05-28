use dioxus::prelude::*;
use crate::component::{other::*, book::*};

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
        #[layout(Content)]
            #[route("/")]
            Home {},
            #[route("/about")]
            About {},
            #[nest("/book")]
                #[route("/")]
                BookIndex {},
                #[route("/:isbn")]
                BookView { isbn: String },
            #[end_nest]
            #[route("/:..route")]
            PageNotFound { route: Vec<String> },
}