#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct TranslationResponse {
    pub(crate) data: TranslationsData,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct TranslationsData {
    pub(crate) translations: Vec<Translation>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Translation {
    pub(crate) translatedText: String,
    pub(crate) detectedSourceLanguage: Option<String>,
}