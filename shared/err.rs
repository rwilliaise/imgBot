use err_context::AnyError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CommandError {
    GenericError(&'static str),
    StringError(String),
    SourcedError(&'static str, AnyError),
    UnhealthyServers,
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("command error: ")?;
        match self {
            CommandError::GenericError(str) => f.write_str(str),
            CommandError::StringError(str) => f.write_str(str.as_str()),
            CommandError::SourcedError(str, e) => {
                f.write_str(format!("{}{}", str, e.to_string()).as_str())
            }
            CommandError::UnhealthyServers => {
                f.write_str("Image servers are unavailable - try again in a few minutes.")
            }
        }
    }
}

impl Error for CommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CommandError::SourcedError(_, e) => Some(&**e),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum ImageError {
    BadRequest(String),
    BadImage(String),
    ProcessingFailure(String),
    FontLoadFailure,
}

impl Display for ImageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("image error: ")?;
        match self {
            ImageError::BadImage(str) => f.write_str(format!("Bad image: {}", str).as_str()),
            ImageError::BadRequest(str) => f.write_str(format!("Bad request: {}", str).as_str()),
            ImageError::ProcessingFailure(str) => {
                f.write_str(format!("Failed to modify image: {}", str).as_str())
            }
            ImageError::FontLoadFailure => f.write_str("Font load failure"),
        }
    }
}

impl Error for ImageError {}

#[derive(Debug)]
pub enum TenorError {
    Unavailable,
    NoProcessed,
    InvalidLink,
    BadResponse(&'static str),
    CannotParse(AnyError),
}

impl Display for TenorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("tenor error: ")?;
        match self {
            TenorError::Unavailable => f.write_str("Tenor API is unavailable"),
            TenorError::NoProcessed => f.write_str("Cannot parse link"),
            TenorError::InvalidLink => f.write_str("Invalid tenor link"),
            TenorError::BadResponse(str) => f.write_str(format!("Bad response: {}", str).as_str()),
            TenorError::CannotParse(e) => f.write_str(format!("Cannot parse response: {:#?}", e).as_str()),
        }
    }
}

impl Error for TenorError {}

