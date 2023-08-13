use fluent_bundle::{bundle, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;

pub type FluentBundle = bundle::FluentBundle<FluentResource, IntlLangMemoizer>;
