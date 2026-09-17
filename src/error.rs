use std::error::Error;
use std::fmt;

pub type AvatarResult<T> = Result<T, AvatarError>;

#[derive(Debug, Clone)]
pub enum AvatarError {
    Io(String),
    Parse(String),
    InvalidFormat(String),
    LoadError(String),
}

impl fmt::Display for AvatarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(message) => write!(f, "I/O error: {message}"),
            Self::Parse(message) => write!(f, "Parse error: {message}"),
            Self::InvalidFormat(message) => write!(f, "Invalid format: {message}"),
            Self::LoadError(message) => write!(f, "Load error: {message}"),
        }
    }
}

impl Error for AvatarError {}

impl From<std::io::Error> for AvatarError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.to_string())
    }
}

impl From<serde_json::Error> for AvatarError {
    fn from(value: serde_json::Error) -> Self {
        Self::Parse(value.to_string())
    }
}
