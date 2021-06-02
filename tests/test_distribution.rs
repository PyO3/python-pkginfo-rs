use python_pkginfo::{Distribution, DistributionType};

#[test]
fn test_parse_wheel() {
    let dist = Distribution::new("tests/fixtures/build-0.4.0-py2.py3-none-any.whl").unwrap();
    assert_eq!(dist.r#type(), DistributionType::Wheel);
    let metadata = dist.metadata();
    assert_eq!(metadata.metadata_version, "2.1");
    assert_eq!(metadata.name, "build");
    assert!(metadata.home_page.is_none());
    assert!(metadata.download_url.is_none());
}

#[test]
fn test_parse_egg() {
    let dist = Distribution::new("tests/fixtures/build-0.4.0-py3.9.egg").unwrap();
    assert_eq!(dist.r#type(), DistributionType::Egg);
    let metadata = dist.metadata();
    assert_eq!(metadata.metadata_version, "2.1");
    assert_eq!(metadata.name, "build");
    assert!(metadata.home_page.is_none());
    assert!(metadata.download_url.is_none());
}
