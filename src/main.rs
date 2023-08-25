use std::net::TcpListener;

use askama::Template;
use axum::{routing::get, Router};
use i18n::Translations;
use listenfd::ListenFd;
use tower_http::services::ServeDir;

mod i18n;
mod log;

#[tokio::main]
async fn main() {
    log::setup();

    i18n::Translator::setup();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .nest_service("/static", ServeDir::new("static"));

    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => listener,
        // otherwise fall back to local listening
        None => TcpListener::bind("127.0.0.1:3000").unwrap(),
    };

    // run our app with hyper
    tracing::info!(
        "Server started, listening on {}",
        listener.local_addr().unwrap()
    );

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root(translations: Translations) -> HelloTemplate {
    HelloTemplate { translations }
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    translations: Translations,
}

mod filters {
    pub use super::i18n::filters::*;
}
