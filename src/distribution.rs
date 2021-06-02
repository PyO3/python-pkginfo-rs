use std::io::{BufReader, Read};
use std::path::Path;

use flate2::read::GzDecoder;
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
        match sdist_type {
            SDistType::Zip => Self::parse_zip(path, "PKG-INFO"),
            SDistType::TarGz => {
                let mut reader =
                    tar::Archive::new(GzDecoder::new(BufReader::new(fs_err::File::open(path)?)));
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
        }
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
            [file1, file2]
                if file1.ends_with(".egg-info/PKG-INFO")
                    || file2.ends_with(".egg-info/PKG-INFO") =>
            {
                let mut buf = Vec::new();
                archive.by_name(file1)?.read_to_end(&mut buf)?;
                Metadata::parse(&buf)
            }
            _ => Err(Error::MultipleMetadataFiles(metadata_files)),
        }
    }
}
