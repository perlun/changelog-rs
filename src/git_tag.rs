use std::cmp::Ordering;
use semver::Version;

#[derive(Clone)]
#[derive(Eq)]
pub struct GitTag {
    pub tag: String,
    pub version: Version
}

impl Ord for GitTag {
    fn cmp(&self, other: &GitTag) -> Ordering {
        self.version.cmp(&other.version)
    }
}

impl PartialOrd for GitTag {
    fn partial_cmp(&self, other: &GitTag) -> Option<Ordering> {
        Some(self.version.cmp(&other.version))
    }
}

impl PartialEq for GitTag {
    fn eq(&self, other: &GitTag) -> bool {
        self.version == other.version
    }
}
