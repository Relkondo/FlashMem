use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct SavedSub {
    pub(crate) original_text: String,
    pub(crate) translated_text: String,
    pub(crate) detected_source_language: String,
}