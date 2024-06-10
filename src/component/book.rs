use dioxus::prelude::*;

use std::env;
use serde::Deserialize;

use crate::component::error::Error;
use crate::component::other::Loading;
use crate::model::book::*;
use crate::route::Route;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    #[serde(rename = "message")]
    image_url: String,
}

#[component]
pub fn BookIndex() -> Element {
    let books = use_resource(get_books);
    
    match &*books.read_unchecked() {
        Some(Ok(data)) => rsx! {
            div {
                class: "flex",
                for book in data {
                    Link {
                        to: Route::BookView { isbn: book.isbn.clone() },
                        BookCard { book: book.clone() }
                    }
                }
            }
            
        },
        Some(Err(e)) => rsx! { Error { error: e.clone() } },
        None => rsx! { Loading {} },
    }
}

#[component]
pub fn BookCard(book: BookEntity) -> Element {
    let image_future = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
    });

    let image_url = match &*image_future.read_unchecked() {
        Some(Ok(response)) => response.image_url.clone(),
        _ => "".to_string(),
    };

    rsx! {
        div {
            class: "max-w-sm rounded overflow-hidden shadow-lg bg-stone-300 dark:bg-slate-600",
            img {
                class: "object-contain h-48",
                src: "{image_url}",
            }
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
pub fn BookView(isbn: ReadOnlySignal<String>) -> Element {
    let future = use_resource(move || async move {
        get_book(isbn()).await
    });

    match &*future.read_unchecked() {
        Some(Ok(book)) => rsx! {
            h1 {
                "{book.title}"
            }
            p {
                "{book.summary}"
            }
        },
        Some(Err(e)) => rsx! { Error {error: e.clone()} },
        None => rsx! { Loading {} }
    }
}

#[server]
pub async fn get_book(isbn: String) -> Result<BookEntity, ServerFnError> {
    let pool = sqlx::postgres::PgPool::connect(crate::config::_DB_URL)
        .await?;

    let book = sqlx::query_as::<_, BookEntity>("SELECT * FROM books WHERE isbn = $1")
        .bind(isbn)
        .fetch_one(&pool)
        .await?;

    Ok(book)
}

#[server]
pub async fn get_books() -> Result<Vec<BookEntity>, ServerFnError> {
    let pool = sqlx::postgres::PgPool::connect(crate::config::_DB_URL)
        .await?;

    let books = sqlx::query_as::<_, BookEntity>("SELECT * FROM books")
        .fetch_all(&pool)
        .await?;

    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
    
    Ok(books)
}

#[server]
pub async fn add_book(book: BookEntity) -> Result<(), ServerFnError> {
    let pool = sqlx::postgres::PgPool::connect(crate::config::_DB_URL)
        .await?;
    
    sqlx::query("INSERT INTO books(isbn, image, title, author, summary) VALUES ($1, $2, $3, $4, $5)")
        .bind(book.isbn)
        .bind(book.image)
        .bind(book.title)
        .bind(book.author)
        .bind(book.summary)
        .execute(&pool)
        .await?;

    Ok(())
}