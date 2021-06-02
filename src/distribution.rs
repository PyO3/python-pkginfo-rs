use std::io::{BufReader, Read};
use std::path::Path;

use zip::ZipArchive;

use crate::{Error, Metadata};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DistributionType {
    SDist,
    Egg,
    Wheel,
}

#[derive(Debug, Clone, Copy)]
enum SDistType {
    Zip,
    TarGz,
}

#[derive(Debug, Clone)]
pub struct Distribution {
    dist_type: DistributionType,
    metadata: Metadata,
}

impl Distribution {
    /// Open and parse a distribution from `path`
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
            let dist_type = match ext {
                "zip" | "gz" => DistributionType::SDist,
                "egg" => DistributionType::Egg,
                "whl" => DistributionType::Wheel,
                _ => return Err(Error::UnknownDistributionType),
            };
            let metadata = match dist_type {
                DistributionType::SDist => {
                    let sdist_type = match ext {
                        "zip" => SDistType::Zip,
                        "gz" => SDistType::TarGz,
                        _ => return Err(Error::UnknownDistributionType),
                    };
                    Self::parse_sdist(path, sdist_type)
                }
                DistributionType::Egg => Self::parse_egg(path),
                DistributionType::Wheel => Self::parse_wheel(path),
            }?;
            return Ok(Self {
                dist_type,
                metadata,
            });
        }
        Err(Error::UnknownDistributionType)
    }

    /// Returns distribution type
    pub fn r#type(&self) -> DistributionType {
        self.dist_type
    }

    /// Returns distribution metadata
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn parse_sdist(path: &Path, sdist_type: SDistType) -> Result<Metadata, Error> {
        todo!()
    }

    fn parse_egg(path: &Path) -> Result<Metadata, Error> {
        Self::parse_zip(path, "EGG-INFO/PKG-INFO")
    }

    fn parse_wheel(path: &Path) -> Result<Metadata, Error> {
        Self::parse_zip(path, ".dist-info/METADATA")
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
            _ => Err(Error::MultipleMetadataFiles(metadata_files)),
        }
    }
}
