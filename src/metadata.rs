use std::str::FromStr;

use mailparse::MailHeaderMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Error;

/// Python package metadata
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Metadata {
    /// Version of the file format; legal values are `1.0`, `1.1`, `1.2`, `2.1` and `2.2`.
    pub metadata_version: String,
    /// The name of the distribution.
    pub name: String,
    /// A string containing the distribution’s version number.
    pub version: String,
    /// A Platform specification describing an operating system supported by the distribution
    /// which is not listed in the “Operating System” Trove classifiers.
    #[cfg_attr(feature = "serde", serde(default))]
    pub platforms: Vec<String>,
    /// Binary distributions containing a PKG-INFO file will use the Supported-Platform field
    /// in their metadata to specify the OS and CPU for which the binary distribution was compiled.
    #[cfg_attr(feature = "serde", serde(default))]
    pub supported_platforms: Vec<String>,
    /// A one-line summary of what the distribution does.
    #[cfg_attr(feature = "serde", serde(default))]
    pub summary: Option<String>,
    /// A longer description of the distribution that can run to several paragraphs.
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    /// A list of additional keywords, separated by commas, to be used to
    /// assist searching for the distribution in a larger catalog.
    #[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Option<String>,
    /// A string containing the URL for the distribution’s home page.
    #[cfg_attr(feature = "serde", serde(default))]
    pub home_page: Option<String>,
    /// A string containing the URL from which this version of the distribution can be downloaded.
    #[cfg_attr(feature = "serde", serde(default))]
    pub download_url: Option<String>,
    /// A string containing the author’s name at a minimum; additional contact information may be provided.
    #[cfg_attr(feature = "serde", serde(default))]
    pub author: Option<String>,
    /// A string containing the author’s e-mail address. It can contain a name and e-mail address in the legal forms for a RFC-822 `From:` header.
    #[cfg_attr(feature = "serde", serde(default))]
    pub author_email: Option<String>,
    /// Text indicating the license covering the distribution where the license is not a selection from the `License` Trove classifiers or an SPDX license expression.
    #[cfg_attr(feature = "serde", serde(default))]
    pub license: Option<String>,
    /// An SPDX expression indicating the license covering the distribution.
    #[cfg_attr(feature = "serde", serde(default))]
    pub license_expression: Option<String>,
    /// Paths to files containing the text of the licenses covering the distribution.
    #[cfg_attr(feature = "serde", serde(default))]
    pub license_files: Vec<String>,
    /// Each entry is a string giving a single classification value for the distribution.
    #[cfg_attr(feature = "serde", serde(default))]
    pub classifiers: Vec<String>,
    /// Each entry contains a string naming some other distutils project required by this distribution.
    #[cfg_attr(feature = "serde", serde(default))]
    pub requires_dist: Vec<String>,
    /// Each entry contains a string naming a Distutils project which is contained within this distribution.
    #[cfg_attr(feature = "serde", serde(default))]
    pub provides_dist: Vec<String>,
    /// Each entry contains a string describing a distutils project’s distribution which this distribution renders obsolete,
    /// meaning that the two projects should not be installed at the same time.
    #[cfg_attr(feature = "serde", serde(default))]
    pub obsoletes_dist: Vec<String>,
    /// A string containing the maintainer’s name at a minimum; additional contact information may be provided.
    ///
    /// Note that this field is intended for use when a project is being maintained by someone other than the original author:
    /// it should be omitted if it is identical to `author`.
    #[cfg_attr(feature = "serde", serde(default))]
    pub maintainer: Option<String>,
    /// A string containing the maintainer’s e-mail address.
    /// It can contain a name and e-mail address in the legal forms for a RFC-822 `From:` header.
    ///
    /// Note that this field is intended for use when a project is being maintained by someone other than the original author:
    /// it should be omitted if it is identical to `author_email`.
    #[cfg_attr(feature = "serde", serde(default))]
    pub maintainer_email: Option<String>,
    /// This field specifies the Python version(s) that the distribution is guaranteed to be compatible with.
    #[cfg_attr(feature = "serde", serde(default))]
    pub requires_python: Option<String>,
    /// Each entry contains a string describing some dependency in the system that the distribution is to be used.
    #[cfg_attr(feature = "serde", serde(default))]
    pub requires_external: Vec<String>,
    /// A string containing a browsable URL for the project and a label for it, separated by a comma.
    #[cfg_attr(feature = "serde", serde(default))]
    pub project_urls: Vec<String>,
    /// A string containing the name of an optional feature. Must be a valid Python identifier.
    /// May be used to make a dependency conditional on whether the optional feature has been requested.
    #[cfg_attr(feature = "serde", serde(default))]
    pub provides_extras: Vec<String>,
    /// A string stating the markup syntax (if any) used in the distribution’s description,
    /// so that tools can intelligently render the description.
    #[cfg_attr(feature = "serde", serde(default))]
    pub description_content_type: Option<String>,
    /// A string containing the name of another core metadata field.
    #[cfg_attr(feature = "serde", serde(default))]
    pub dynamic: Vec<String>,
}

impl Metadata {
    /// Parse distribution metadata from metadata bytes
    pub fn parse(content: &[u8]) -> Result<Self, Error> {
        // HACK: trick mailparse to parse as UTF-8 instead of ASCII
        let mut mail = b"Content-Type: text/plain; charset=utf-8\n".to_vec();
        mail.extend_from_slice(content);

        let msg = mailparse::parse_mail(&mail)?;
        let headers = msg.get_headers();
        let get_first_value = |name| {
            headers.get_first_header(name).and_then(|header| {
                match rfc2047_decoder::decode(header.get_value_raw()) {
                    Ok(value) => {
                        if value == "UNKNOWN" {
                            None
                        } else {
                            Some(value)
                        }
                    }
                    Err(_) => None,
                }
            })
        };
        let get_all_values = |name| {
            let values: Vec<String> = headers
                .get_all_values(name)
                .into_iter()
                .filter(|value| value != "UNKNOWN")
                .collect();
            values
        };
        let metadata_version = headers
            .get_first_value("Metadata-Version")
            .ok_or(Error::FieldNotFound("Metadata-Version"))?;
        let name = headers
            .get_first_value("Name")
            .ok_or(Error::FieldNotFound("Name"))?;
        let version = headers
            .get_first_value("Version")
            .ok_or(Error::FieldNotFound("Version"))?;
        let platforms = get_all_values("Platform");
        let supported_platforms = get_all_values("Supported-Platform");
        let summary = get_first_value("Summary");
        let body = msg.get_body()?;
        let description = if !body.trim().is_empty() {
            Some(body)
        } else {
            get_first_value("Description")
        };
        let keywords = get_first_value("Keywords");
        let home_page = get_first_value("Home-Page");
        let download_url = get_first_value("Download-URL");
        let author = get_first_value("Author");
        let author_email = get_first_value("Author-email");
        let license = get_first_value("License");
        let license_expression = get_first_value("License-Expression");
        let license_files = get_all_values("License-File");
        let classifiers = get_all_values("Classifier");
        let requires_dist = get_all_values("Requires-Dist");
        let provides_dist = get_all_values("Provides-Dist");
        let obsoletes_dist = get_all_values("Obsoletes-Dist");
        let maintainer = get_first_value("Maintainer");
        let maintainer_email = get_first_value("Maintainer-email");
        let requires_python = get_first_value("Requires-Python");
        let requires_external = get_all_values("Requires-External");
        let project_urls = get_all_values("Project-URL");
        let provides_extras = get_all_values("Provides-Extra");
        let description_content_type = get_first_value("Description-Content-Type");
        let dynamic = get_all_values("Dynamic");
        Ok(Metadata {
            metadata_version,
            name,
            version,
            platforms,
            supported_platforms,
            summary,
            description,
            keywords,
            home_page,
            download_url,
            author,
            author_email,
            license,
            license_expression,
            license_files,
            classifiers,
            requires_dist,
            provides_dist,
            obsoletes_dist,
            maintainer,
            maintainer_email,
            requires_python,
            requires_external,
            project_urls,
            provides_extras,
            description_content_type,
            dynamic,
        })
    }
}

impl FromStr for Metadata {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Metadata::parse(s.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::Metadata;
    use crate::Error;

    #[test]
    fn test_parse_from_str() {
        let s = "Metadata-Version: 1.0";
        let meta: Result<Metadata, Error> = s.parse();
        assert!(matches!(meta, Err(Error::FieldNotFound("Name"))));

        let s = "Metadata-Version: 1.0\nName: asdf";
        let meta = Metadata::parse(s.as_bytes());
        assert!(matches!(meta, Err(Error::FieldNotFound("Version"))));

        let s = "Metadata-Version: 1.0\nName: asdf\nVersion: 1.0";
        let meta = Metadata::parse(s.as_bytes()).unwrap();
        assert_eq!(meta.metadata_version, "1.0");
        assert_eq!(meta.name, "asdf");
        assert_eq!(meta.version, "1.0");

        let s = "Metadata-Version: 1.0\nName: asdf\nVersion: 1.0\nDescription: a Python package";
        let meta: Metadata = s.parse().unwrap();
        assert_eq!(meta.description.as_deref(), Some("a Python package"));

        let s = "Metadata-Version: 1.0\nName: asdf\nVersion: 1.0\n\na Python package";
        let meta: Metadata = s.parse().unwrap();
        assert_eq!(meta.description.as_deref(), Some("a Python package"));

        let s = "Metadata-Version: 1.0\nName: asdf\nVersion: 1.0\nAuthor: 中文\n\n一个 Python 包";
        let meta: Metadata = s.parse().unwrap();
        assert_eq!(meta.author.as_deref(), Some("中文"));
        assert_eq!(meta.description.as_deref(), Some("一个 Python 包"));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_deserialize() {
        let input = r#"{"metadata_version": "2.3", "name": "example", "version": "1.0.0"}"#;
        let _metadata: Metadata = serde_json::from_str(input).unwrap();
    }
}
