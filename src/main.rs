#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

mod component;
mod model;
mod route;
mod config;

use crate::route::Route;

const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    #[cfg(feature = "web")]
    dioxus_web::launch::launch_cfg(App, dioxus_web::Config::new().hydrate(true));

    #[cfg(feature = "server")]
    {
        use axum::Router;

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let app = Router::new()
                    .serve_dioxus_application(ServeConfig::builder().build(), || {
                        VirtualDom::new(App)
                    })
                    .await;

                let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
                    .await
                    .unwrap();

                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();
            });
    } 
}

fn App() -> Element {
    rsx! {
        div {
            class: "dark h-full text-slate-500 dark:text-slate-400 bg-white dark:bg-slate-900",
            Router::<Route> {}
        }
    }
}