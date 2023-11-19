use askama::Template;
use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use serde::Deserialize;
use tower_http::services::ServeDir;
use std::net::SocketAddr;

pub use self::error::{Error, Result};

mod error;

// -- region: reg_statics

// -- endregion: reg_statics


// -- region: reg_structs --
#[derive(Template)]
#[template(path = "index.html")]
struct BaseTemplate<'a> {
    name: &'a str,
    title: &'a str,
}

#[derive(Deserialize)]
struct QueryParams {
    name: Option<String>,
}

// -- endregion: reg_structs --

// -- region: reg_main

#[tokio::main]
async fn main() -> Result<()> {
    let routes = Router::new()
        .route("/", get(index))
        .route("/test", get(test))
        .nest_service("/dist", ServeDir::new("assets"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("-->> {:<12} on {}", "LISTENING", addr);

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    return Ok(());
}

// -- endregion: reg_main

// -- region: reg_handers

async fn index() -> impl IntoResponse {
    println!("-->> {:<12} - index", "HANDLER");
    return Html(format!("Hello, <strong>World</strong>!!!"));
}

async fn test(Query(params): Query<QueryParams>) -> impl IntoResponse {
    println!("-->> {:<12} - test", "HANDLER");

    let _name = params.name.as_deref().unwrap_or("World!!");
    let hello = BaseTemplate {
        name: &_name,
        title: "Hello",
    };

    return Html(hello.render().unwrap());
}

// -- region: reg_handers
