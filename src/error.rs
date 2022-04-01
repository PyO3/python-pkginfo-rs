use std::{error, fmt, io};

use mailparse::MailParseError;
use zip::result::ZipError;

/// The error type
#[derive(Debug)]
pub enum Error {
    /// I/O error
    Io(io::Error),
    /// mail parse error
    MailParse(MailParseError),
    /// Zip parse error
    Zip(ZipError),
    /// Metadata field not found
    FieldNotFound(&'static str),
    /// Unknown distribution type
    UnknownDistributionType,
    /// Metadata file not found
    MetadataNotFound,
    /// Multiple metadata files found
    MultipleMetadataFiles(Vec<String>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => err.fmt(f),
            Error::MailParse(err) => err.fmt(f),
            Error::Zip(err) => err.fmt(f),
            Error::FieldNotFound(key) => write!(f, "metadata field {} not found", key),
            Error::UnknownDistributionType => write!(f, "unknown distribution type"),
            Error::MetadataNotFound => write!(f, "metadata file not found"),
            Error::MultipleMetadataFiles(files) => {
                write!(f, "found multiple metadata files: {:?}", files)
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(err) => err.source(),
            Error::MailParse(err) => err.source(),
            Error::Zip(err) => err.source(),
            Error::FieldNotFound(_)
            | Error::UnknownDistributionType
            | Error::MetadataNotFound
            | Error::MultipleMetadataFiles(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<MailParseError> for Error {
    fn from(err: MailParseError) -> Self {
        Self::MailParse(err)
    }
}

impl From<ZipError> for Error {
    fn from(err: ZipError) -> Self {
        Self::Zip(err)
    }
}
