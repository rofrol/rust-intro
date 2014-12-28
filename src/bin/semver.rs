extern crate semver;
use semver::Version;

fn main() {
    assert!(Version::parse("1.2.3") == Ok(Version {
        major: 1,
        minor: 2,
        patch: 3,
        pre: vec!(),
        build: vec!()
    }));
}
