use std::{error, fmt, io};

use mailparse::MailParseError;

/// The error type
#[derive(Debug)]
pub enum Error {
    /// mail parse error
    MailParse(MailParseError),
    /// Metadata field not found
    FieldNotFound(&'static str),
    /// I/O error
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MailParse(err) => err.fmt(f),
            Error::FieldNotFound(key) => write!(f, "metadata field {} not found", key),
            Error::Io(err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::MailParse(err) => Some(err),
            Error::FieldNotFound(_) => None,
            Error::Io(err) => Some(err),
        }
    }
}

impl From<MailParseError> for Error {
    fn from(err: MailParseError) -> Self {
        Self::MailParse(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
