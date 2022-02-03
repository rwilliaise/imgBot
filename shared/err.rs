use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CommandError {
    GenericError(&'static str),
    UnhealthyServers,
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::GenericError(str) => f.write_str(str),
            CommandError::UnhealthyServers => f.write_str("Image servers are unavailable - try again in a few minutes."),
        }
    }
}

impl Error for CommandError {}
