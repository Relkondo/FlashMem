use tesseract::plumbing::leptonica_plumbing::PixReadMemError;
use tesseract::plumbing::{TessBaseApiGetUtf8TextError, TessBaseApiRecogniseError};
#[derive(Debug)]
pub(crate) enum TesseractOriginatedError {
    PixReadMemError(PixReadMemError),
    TessBaseApiRecogniseError(TessBaseApiRecogniseError),
    TessBaseApiGetUtf8TextError(TessBaseApiGetUtf8TextError)
}

impl From<PixReadMemError> for TesseractOriginatedError {
    fn from(err: PixReadMemError) -> TesseractOriginatedError {
        TesseractOriginatedError::PixReadMemError(err)
    }
}

impl From<TessBaseApiRecogniseError> for TesseractOriginatedError {
    fn from(err: TessBaseApiRecogniseError) -> TesseractOriginatedError {
        TesseractOriginatedError::TessBaseApiRecogniseError(err)
    }
}


impl From<TessBaseApiGetUtf8TextError> for TesseractOriginatedError {
    fn from(err: TessBaseApiGetUtf8TextError) -> TesseractOriginatedError {
        TesseractOriginatedError::TessBaseApiGetUtf8TextError(err)
    }
}