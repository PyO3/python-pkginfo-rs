use std::str::FromStr;

use crate::Error;
use mailparse::MailHeaderMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Distribution {
    /// Version of the file format; legal values are “1.0”, “1.1”, “1.2”, “2.1” and “2.2”.
    pub metadata_version: String,
    /// The name of the distribution.
    pub name: String,
    /// A string containing the distribution’s version number.
    pub version: String,
    /// A Platform specification describing an operating system supported by the distribution
    /// which is not listed in the “Operating System” Trove classifiers.
    pub platforms: Vec<String>,
    /// Binary distributions containing a PKG-INFO file will use the Supported-Platform field
    /// in their metadata to specify the OS and CPU for which the binary distribution was compiled.
    pub supported_platforms: Vec<String>,
    /// A one-line summary of what the distribution does.
    pub summary: Option<String>,
    /// A longer description of the distribution that can run to several paragraphs.
    pub description: Option<String>,
    /// A list of additional keywords, separated by commas, to be used to
    /// assist searching for the distribution in a larger catalog.
    pub keywords: Vec<String>,
    /// A string containing the URL for the distribution’s home page.
    pub home_page: Option<String>,
    /// A string containing the URL from which this version of the distribution can be downloaded.
    pub download_url: Option<String>,
    /// A string containing the author’s name at a minimum; additional contact information may be provided.
    pub author: Option<String>,
    /// A string containing the author’s e-mail address. It can contain a name and e-mail address in the legal forms for a RFC-822 `From:` header.
    pub author_email: Option<String>,
    /// Text indicating the license covering the distribution where the license is not a selection from the “License” Trove classifiers.
    pub license: Option<String>,
    /// Each entry is a string giving a single classification value for the distribution.
    pub classifiers: Vec<String>,
    /// Each entry contains a string naming some other distutils project required by this distribution.
    pub requires_dist: Vec<String>,
    /// Each entry contains a string naming a Distutils project which is contained within this distribution.
    pub provides_dist: Vec<String>,
    /// Each entry contains a string describing a distutils project’s distribution which this distribution renders obsolete,
    /// meaning that the two projects should not be installed at the same time.
    pub obsoletes_dist: Vec<String>,
    /// A string containing the maintainer’s name at a minimum; additional contact information may be provided.
    ///
    /// Note that this field is intended for use when a project is being maintained by someone other than the original author:
    /// it should be omitted if it is identical to `author`.
    pub maintainer: Option<String>,
    /// A string containing the maintainer’s e-mail address.
    /// It can contain a name and e-mail address in the legal forms for a RFC-822 `From:` header.
    ///
    /// Note that this field is intended for use when a project is being maintained by someone other than the original author:
    /// it should be omitted if it is identical to `author_email`.
    pub maintainer_email: Option<String>,
    /// This field specifies the Python version(s) that the distribution is guaranteed to be compatible with.
    pub requires_python: Option<String>,
    /// Each entry contains a string describing some dependency in the system that the distribution is to be used.
    pub requires_external: Vec<String>,
    /// A string containing a browsable URL for the project and a label for it, separated by a comma.
    pub project_urls: Vec<String>,
    /// A string containing the name of an optional feature. Must be a valid Python identifier.
    /// May be used to make a dependency conditional on whether the optional feature has been requested.
    pub provides_extras: Vec<String>,
    /// A string stating the markup syntax (if any) used in the distribution’s description,
    /// so that tools can intelligently render the description.
    pub description_content_type: Option<String>,
}

impl FromStr for Distribution {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let msg = mailparse::parse_mail(s.as_bytes())?;
        let headers = msg.get_headers();
        let metadata_version = headers
            .get_first_value("Metadata-Version")
            .ok_or_else(|| Error::KeyError("Metadata-Version"))?;
        Ok(Distribution {
            metadata_version,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Distribution;

    #[test]
    fn test_parse_from_str() {
        let s = "Metadata-Version: 1.0";
        let dist: Distribution = s.parse().unwrap();
        assert_eq!(dist.metadata_version, "1.0");
    }
}
