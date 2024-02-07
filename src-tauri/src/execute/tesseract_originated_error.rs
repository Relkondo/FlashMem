use tesseract::InitializeError;
use tesseract::plumbing::leptonica_plumbing::PixReadMemError;
use tesseract::plumbing::{TessBaseApiGetUtf8TextError, TessBaseApiRecogniseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TesseractOriginatedError {
    #[error(transparent)]
    InitializeError(#[from] InitializeError),
    #[error(transparent)]
    DecodeError(#[from] image::ImageError),
    #[error(transparent)]
    SendImageError(#[from] PixReadMemError),
    #[error(transparent)]
    RecognizeError(#[from] TessBaseApiRecogniseError),
    #[error(transparent)]
    TextScanError(#[from] TessBaseApiGetUtf8TextError),
}

