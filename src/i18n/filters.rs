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
