use std::fmt;
use std::io::{BufReader, Read};
use std::path::Path;
use std::str::FromStr;

#[cfg(feature = "bzip2")]
use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
#[cfg(feature = "xz")]
use xz::bufread::XzDecoder;
#[cfg(feature = "xz")]
use xz::stream::Stream as XzStream;
use zip::ZipArchive;

use crate::{Error, Metadata};

/// Python package distribution type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistributionType {
    /// Source distribution
    SDist,
    /// Binary distribution egg format
    Egg,
    /// Binary distribution wheel format
    Wheel,
}

#[derive(Debug, Clone, Copy)]
enum SDistType {
    Zip,
    GzTar,
    #[cfg(feature = "deprecated-formats")]
    Tar,
    #[cfg(feature = "bzip2")]
    BzTar,
    #[cfg(feature = "xz")]
    XzTar,
}

/// Python package distribution
#[derive(Debug, Clone)]
pub struct Distribution {
    dist_type: DistributionType,
    metadata: Metadata,
    python_version: String,
}

impl fmt::Display for DistributionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DistributionType::SDist => write!(f, "sdist"),
            DistributionType::Egg => write!(f, "bdist_egg"),
            DistributionType::Wheel => write!(f, "bdist_wheel"),
        }
    }
}

impl FromStr for SDistType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dist_type = match s {
            "zip" => SDistType::Zip,
            "gz" | "tgz" => SDistType::GzTar,
            #[cfg(feature = "deprecated-formats")]
            "tar" => SDistType::Tar,
            #[cfg(feature = "bzip2")]
            "bz2" | "tbz" => SDistType::BzTar,
            #[cfg(feature = "xz")]
            "lz" | "lzma" | "tlz" | "txz" | "xz" => SDistType::XzTar,
            _ => return Err(Error::UnknownDistributionType),
        };
        Ok(dist_type)
    }
}

impl Distribution {
    /// Open and parse a distribution from `path`
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let ext = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or(Error::UnknownDistributionType)?;

        Ok(if let Ok(sdist_type) = ext.parse() {
            Self {
                dist_type: DistributionType::SDist,
                metadata: Self::parse_sdist(path, sdist_type)?,
                python_version: "source".to_string(),
            }
        } else {
            match ext {
                "egg" => {
                    let parts: Vec<&str> = path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .split('-')
                        .collect();
                    let python_version = match parts.as_slice() {
                        [_name, _version, py_ver] => py_ver,
                        _ => "any",
                    };
                    Self {
                        dist_type: DistributionType::Egg,
                        metadata: Self::parse_egg(path)?,
                        python_version: python_version.to_string(),
                    }
                }
                "whl" => {
                    let parts: Vec<&str> = path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .split('-')
                        .collect();
                    let python_version = match parts.as_slice() {
                        [_name, _version, py_ver, _abi_tag, _plat_tag] => py_ver,
                        _ => "any",
                    };
                    Self {
                        dist_type: DistributionType::Wheel,
                        metadata: Self::parse_wheel(path)?,
                        python_version: python_version.to_string(),
                    }
                }
                _ => return Err(Error::UnknownDistributionType),
            }
        })
    }

    /// Returns distribution type
    pub fn r#type(&self) -> DistributionType {
        self.dist_type
    }

    /// Returns distribution metadata
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Returns the supported Python version tag
    ///
    /// For source distributions the version tag is always `source`
    pub fn python_version(&self) -> &str {
        &self.python_version
    }

    fn parse_sdist(path: &Path, sdist_type: SDistType) -> Result<Metadata, Error> {
        match sdist_type {
            SDistType::Zip => Self::parse_zip(path, "PKG-INFO"),
            SDistType::GzTar => {
                Self::parse_tar(GzDecoder::new(BufReader::new(fs_err::File::open(path)?)))
            }
            #[cfg(feature = "deprecated-formats")]
            SDistType::Tar => Self::parse_tar(BufReader::new(fs_err::File::open(path)?)),
            #[cfg(feature = "bzip2")]
            SDistType::BzTar => {
                Self::parse_tar(BzDecoder::new(BufReader::new(fs_err::File::open(path)?)))
            }
            #[cfg(feature = "xz")]
            SDistType::XzTar => Self::parse_tar(XzDecoder::new_stream(
                BufReader::new(fs_err::File::open(path)?),
                XzStream::new_auto_decoder(u64::MAX, 0).unwrap(),
            )),
        }
    }

    fn parse_egg(path: &Path) -> Result<Metadata, Error> {
        Self::parse_zip(path, "EGG-INFO/PKG-INFO")
    }

    fn parse_wheel(path: &Path) -> Result<Metadata, Error> {
        Self::parse_zip(path, ".dist-info/METADATA")
    }

    fn parse_tar<R: Read>(reader: R) -> Result<Metadata, Error> {
        let mut reader = tar::Archive::new(reader);
        let metadata_file = reader
            .entries()?
            .map(|entry| -> Result<_, Error> {
                let entry = entry?;
                if entry.path()?.ends_with("PKG-INFO") {
                    Ok(Some(entry))
                } else {
                    Ok(None)
                }
            })
            .find_map(|x| x.transpose());
        if let Some(metadata_file) = metadata_file {
            let mut entry = metadata_file?;
            let mut buf = Vec::new();
            entry.read_to_end(&mut buf)?;
            Metadata::parse(&buf)
        } else {
            Err(Error::MetadataNotFound)
        }
    }

    fn parse_zip(path: &Path, metadata_file_suffix: &str) -> Result<Metadata, Error> {
        let reader = BufReader::new(fs_err::File::open(path)?);
        let mut archive = ZipArchive::new(reader)?;
        let metadata_files: Vec<_> = archive
            .file_names()
            .filter(|name| name.ends_with(metadata_file_suffix))
            .map(ToString::to_string)
            .collect();
        match metadata_files.as_slice() {
            [] => Err(Error::MetadataNotFound),
            [metadata_file] => {
                let mut buf = Vec::new();
                archive.by_name(metadata_file)?.read_to_end(&mut buf)?;
                Metadata::parse(&buf)
            }
            [file1, file2]
                if file1.ends_with(".egg-info/PKG-INFO")
                    || file2.ends_with(".egg-info/PKG-INFO") =>
            {
                let mut buf = Vec::new();
                archive.by_name(file1)?.read_to_end(&mut buf)?;
                Metadata::parse(&buf)
            }
            _ => {
                let top_level_files: Vec<_> = metadata_files
                    .iter()
                    .filter(|f| {
                        let path = Path::new(f);
                        path.components().count() == 2
                    })
                    .collect();
                if top_level_files.len() == 1 {
                    let mut buf = Vec::new();
                    archive.by_name(top_level_files[0])?.read_to_end(&mut buf)?;
                    return Metadata::parse(&buf);
                }
                Err(Error::MultipleMetadataFiles(metadata_files))
            }
        }
    }
}
