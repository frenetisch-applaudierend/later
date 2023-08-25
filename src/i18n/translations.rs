use axum::{extract::FromRequestParts, http::request};
use unic_langid::LanguageIdentifier;

use super::{FluentBundle, Translator, UserLanguage};

pub struct Translations {
    pub lang: LanguageIdentifier,
    pub bundle: &'static FluentBundle,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for Translations
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let UserLanguage(lang) = UserLanguage::from_request_parts(parts, state).await?;
        let bundle = Translator::get_lang(&lang);

        Ok(Translations { lang, bundle })
    }
}
