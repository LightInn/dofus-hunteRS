use ocrs::ImageSourceError;
use thiserror::Error;
use xcap::XCapError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("API error: {0}")]
    Api(#[from] ApiError),
    #[error("Capture error: {0}")]
    Capture(#[from] CaptureError),
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    #[error("Model load error: {0}")]
    ModelLoad(#[from] rten::ModelLoadError),
    #[error("Anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),
    #[error("Image source error: {0}")] // Nouvelle variante pour ImageSourceError
    ImageSource(#[from] ImageSourceError),
    #[error("IO error: {0}")] // Nouvelle variante pour std::io::Error
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Invalid header: {0}")]
    InvalidHeader(String),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum CaptureError {
    #[error("Monitor not found")]
    MonitorNotFound,
    #[error("Invalid region")]
    InvalidRegion,
    #[error("Capture failed: {0}")]
    CaptureFailed(#[from] XCapError),
    #[error("Unknown region: {0}")]
    UnknownRegion(String),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
