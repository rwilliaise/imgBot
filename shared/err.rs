use std::error::Error;
use std::fmt::{Display, Formatter};
use err_context::AnyError;

#[derive(Debug)]
pub enum CommandError {
    GenericError(&'static str),
    StringError(String),
    SourcedError(&'static str, AnyError),
    UnhealthyServers,
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
