use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("io error : {0}")]
    IO(#[from] std::io::Error),
    #[error("serde_json error : {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("JavaScript error ")]
    JavaScript,
    #[error("error : {0}")]
    Message(String),
    #[error("none value get")]
    None,

    #[error("error : {0}")]
    BskySdk(#[from] bsky_sdk::Error),

    #[error("opendal error : {0}")]
    Opendal(#[from] opendal::Error),

    #[error("reqwest error : {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("anyhow error : {0}")]
    Anyhow(#[from] anyhow::Error),
}

pub trait IntoCustomError<T> {
    fn into_custom_error(self) -> Result<T, CustomError>;
}

impl<T> IntoCustomError<T> for Option<T> {
    fn into_custom_error(self) -> Result<T, CustomError> {
        if let Some(this) = self {
            Ok(this)
        } else {
            Err(CustomError::None)
        }
    }
}

impl IntoCustomError<()> for String {
    fn into_custom_error(self) -> Result<(), CustomError> {
        Err(CustomError::Message(self))
    }
}
