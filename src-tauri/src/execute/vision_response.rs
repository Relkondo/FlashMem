#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct VisionResponse {
    pub(crate) responses: Vec<AnnotateImageResponse>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct AnnotateImageResponse {
    pub(crate) textAnnotations: Vec<TextAnnotation>,
    pub(crate) fullTextAnnotation: Option<FullTextAnnotation>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct FullTextAnnotation {
    pub(crate) pages: Vec<Page>,
    pub(crate) text: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Page {
    pub(crate) blocks: Vec<Block>,
    pub(crate) property: Option<TextProperty>,
    pub(crate) width: i32,
    pub(crate) height: i32,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Block {
    pub(crate) paragraphs: Vec<Paragraph>,
    pub(crate) blockType: String,
    pub(crate) boundingBox: Option<BoundingPoly>,

}

#[derive(Deserialize, Serialize)]
pub(crate) struct Paragraph {
    pub(crate) words: Vec<Word>,
    pub(crate) boundingBox: Option<BoundingPoly>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Word {
    pub(crate) symbols: Vec<Symbol>,
    pub(crate) property: Option<TextProperty>,
    pub(crate) boundingBox: Option<BoundingPoly>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Symbol {
    pub(crate) text: String,
    pub(crate) property: Option<TextProperty>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct TextProperty {
    pub(crate) detectedLanguages: Option<Vec<DetectedLanguage>>,
    pub(crate) detectedBreak: Option<DetectedBreak>,
    pub(crate) detectedOrientation: Option<String>,

}

#[derive(Deserialize, Serialize)]
pub(crate) struct DetectedBreak {
    #[serde(rename = "type")]
    pub(crate) type_: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct DetectedLanguage {
    pub(crate) languageCode: String,
    pub(crate) confidence: f32,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct TextAnnotation {
    pub(crate) description: String,
    pub(crate) locale: Option<String>,
    pub(crate) boundingPoly: Option<BoundingPoly>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct BoundingPoly {
    pub(crate) vertices: Vec<Vertex>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Vertex {
    pub(crate) x: i32,
    pub(crate) y: i32,
}