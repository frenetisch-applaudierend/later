use super::Translations;

pub fn fluent(val: &str, lang: &Translations) -> askama::Result<String> {
    let lang = lang.bundle;
    let msg = lang.get_message(val).expect("Could not read message");
    let pattern = msg.value().expect("Could not read value");
    let mut errors = vec![];
    let value = lang.format_pattern(pattern, None, &mut errors);

    for error in errors {
        tracing::error!("Error: {}", error);
    }
    return Ok(value.to_string());
}
