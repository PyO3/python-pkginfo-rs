use std::io;

use mailparse::MailParseError;
use thiserror::Error;
use zip::result::ZipError;

/// The error type
#[derive(Error, Debug)]
pub enum Error {
    /// I/O error
    #[error(transparent)]
    Io(#[from] io::Error),
    /// mail parse error
    #[error(transparent)]
    MailParse(#[from] MailParseError),
    /// Zip parse error
    #[error(transparent)]
    Zip(#[from] ZipError),
    /// Metadata field not found
    #[error("metadata field {0} not found")]
    FieldNotFound(&'static str),
    /// Unknown distribution type
    #[error("unknown distribution type")]
    UnknownDistributionType,
    /// Metadata file not found
    #[error("metadata file not found")]
    MetadataNotFound,
    /// Multiple metadata files found
    #[error("found multiple metadata files: {0:?}")]
    MultipleMetadataFiles(Vec<String>),
}
