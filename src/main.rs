use std::{net::SocketAddr, sync::OnceLock};

use askama::Template;
use axum::{routing::get, Router};
use fluent_bundle::{bundle::FluentBundle, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    _ = TRANSLATOR.set(i18n::Translator::new());

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
    let lang = &TRANSLATOR.get().unwrap().en;
    HelloTemplate { lang }
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    lang: &'a FluentBundle<FluentResource, IntlLangMemoizer>,
}

static TRANSLATOR: OnceLock<i18n::Translator> = OnceLock::new();

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

mod i18n {

    use fluent_bundle::{bundle::FluentBundle, FluentResource};
    use intl_memoizer::concurrent::IntlLangMemoizer;
    use unic_langid::langid;

    pub struct Translator {
        pub en: FluentBundle<FluentResource, IntlLangMemoizer>,
    }

    impl Translator {
        pub fn new() -> Self {
            let langid_en = langid!("en");
            let mut bundle_en = FluentBundle::new_concurrent(vec![langid_en]);

            let res_en = FluentResource::try_new("hello-world = Hello, World!".to_string())
                .expect("What could fail?");

            bundle_en
                .add_resource(res_en)
                .expect("Why would this fail?");

            Translator { en: bundle_en }
        }
    }
}
