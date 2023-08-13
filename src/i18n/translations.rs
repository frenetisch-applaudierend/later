use axum::{extract::FromRequestParts, http::request};

use super::{FluentBundle, Translator, UserLanguage};

pub struct Translations(pub &'static FluentBundle);

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

        Ok(Translations(Translator::get_lang(&lang)))
    }
}
