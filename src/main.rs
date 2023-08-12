use std::{collections::HashMap, net::SocketAddr};

use askama::Template;
use axum::{extract::Query, routing::get, Router};
use fluent_bundle::{bundle::FluentBundle, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;

mod i18n;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    i18n::Translator::setup();

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
async fn root(Query(params): Query<HashMap<String, String>>) -> HelloTemplate<'static> {
    let lang = params.get("lang").map_or("en", |s| s.as_str());

    tracing::info!("Language: {}", lang);

    let lang = i18n::Translator::get_lang(lang);
    HelloTemplate { lang }
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    lang: &'a FluentBundle<FluentResource, IntlLangMemoizer>,
}

mod filters {
    use fluent_bundle::{bundle::FluentBundle, FluentResource};
    use intl_memoizer::concurrent::IntlLangMemoizer;

    pub fn fluent(
        val: &str,
        lang: &FluentBundle<FluentResource, IntlLangMemoizer>,
    ) -> askama::Result<String> {
        let msg = lang.get_message(val).expect("Could not read message");
        let pattern = msg.value().expect("Could not read value");
        let mut errors = vec![];
        let value = lang.format_pattern(pattern, None, &mut errors);

        for error in errors {
            tracing::error!("Error: {}", error);
        }
        return Ok(value.to_string());
    }
}
