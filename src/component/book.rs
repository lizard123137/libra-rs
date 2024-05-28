use dioxus::prelude::*;

use std::env;

use crate::component::other::{Loading, Error};
use crate::model::book::*;
use crate::route::Route;

#[component]
pub fn BookIndex() -> Element {
    let books = use_resource(get_books);
    
    match &*books.read_unchecked() {
        Some(Ok(data)) => rsx! {
            for book in data {
                Link {
                    to: Route::BookView { isbn: book.isbn.clone() },
                    BookCard { book: book.clone() }
                }
            }
        },
        Some(Err(_)) => rsx! { Error {} },
        None => rsx! { Loading {} },
    }
}

#[component]
pub fn BookCard(book: BookEntity) -> Element {
    rsx! {
        div {
            class: "max-w-sm rounded overflow-hidden shadow-lg bg-stone-300 dark:bg-slate-600",
            h1 {
                class: "font-bold text-xl mb-2",
                "{book.title}"
            }
            h2 {
                "By {book.author}"
            }
            p {
                class: "text-gray-700 text-base line-clamp-3",
                "{book.summary}"
            }
        }
    }
}

#[component]
pub fn BookView(isbn: String) -> Element {
    rsx! { 
        h2 {
            "Book of isbn: {isbn}"
        }
    }
}

#[server]
async fn get_books() -> Result<Vec<BookEntity>, ServerFnError> {
    let url = "postgres://postgres:password@localhost/librars";
    let pool = sqlx::postgres::PgPool::connect(&url)
        .await
        .expect("Database connection failed!");

    let books = sqlx::query_as::<_, BookEntity>("SELECT * FROM books")
        .fetch_all(&pool)
        .await?;

    tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
    
    Ok(books)
}