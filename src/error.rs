use std::{error, fmt};

use mailparse::MailParseError;

/// The error type
#[derive(Debug)]
pub enum Error {
    /// mail parse error
    MailParse(MailParseError),
    /// Metadata key error
    KeyError(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MailParse(err) => err.fmt(f),
            Error::KeyError(key) => write!(f, "metadata key {} not found", key),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::MailParse(err) => Some(err),
            Error::KeyError(_) => None,
        }
    }
}

impl From<MailParseError> for Error {
    fn from(err: MailParseError) -> Self {
        Self::MailParse(err)
    }
}
