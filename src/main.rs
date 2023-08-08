use std::net::SocketAddr;

use askama::Template;
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> HelloTemplate<'static> {
    HelloTemplate { name: "Dolly" }
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}
