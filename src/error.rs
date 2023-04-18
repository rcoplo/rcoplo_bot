use thiserror::Error;

#[derive(Error,Debug)]
pub enum BotError {

    #[error("other error {0}")]
    Other(String),

    #[error("RcoploBot error {0}")]
    RcoploBotError(String),

    #[error("timeout error")]
    Timeout,

    #[error("network error")]
    Network,

    #[error("io error {0}")]
    IO(#[from] std::io::Error),

    #[error("http get error {0}")]
    HttpGetError(#[from] reqwest::Error),

    #[error("json error {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("base64 decode error {0}")]
    Base64Error(#[from] base64::DecodeError),

    #[error("yaml error {0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("rb error {0}")]
    RbError(#[from] rbatis::Error),
}


pub type BotResult<T> = Result<T, BotError>;

impl From<&str> for BotError{
    fn from(value: &str) -> Self {
        BotError::RcoploBotError(value.to_string())
    }
}

impl From<String> for BotError{
    fn from(value: String) -> Self {
        BotError::RcoploBotError(value)
    }
}

