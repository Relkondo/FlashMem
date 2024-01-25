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
    #[allow(non_snake_case)]pub(crate) translatedText: String,
    #[allow(non_snake_case)]pub(crate) detectedSourceLanguage: Option<String>,
}