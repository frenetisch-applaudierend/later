use std::collections::HashMap;

use axum::{
    extract::{FromRequestParts, Query},
    http::request,
    RequestPartsExt,
};
use unic_langid::{langid, LanguageIdentifier};

pub struct UserLanguage(pub LanguageIdentifier);

#[axum::async_trait]
impl<S> FromRequestParts<S> for UserLanguage
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        if let Some(lang) = Self::read_from_query(parts).await {
            return Ok(UserLanguage(lang));
        }

        Ok(UserLanguage(langid!("en")))
    }
}

impl UserLanguage {
    async fn read_from_query(parts: &mut request::Parts) -> Option<LanguageIdentifier> {
        let Query(query) = parts
            .extract::<Query<HashMap<String, String>>>()
            .await
            .ok()?;
        let lang = query.get("lang").map(|s| s.as_str())?;
        lang.parse().ok()
    }
}
