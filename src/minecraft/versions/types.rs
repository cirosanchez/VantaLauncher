use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VersionManifest {
    pub latest: LatestVersions,
    pub versions: Vec<VersionInfo>,
}

#[derive(Debug, Deserialize)]
pub struct LatestVersions {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Deserialize)]
pub struct VersionInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: VersionType,
    pub url: String,
    pub sha1: String,
}

#[derive(Debug, Deserialize)]
pub struct VersionMetadata {
    pub id: String,

    #[serde(rename = "mainClass")]
    pub main_class: String,

    pub downloads: Downloads,

    pub libraries: Vec<Library>,
}

#[derive(Debug, Deserialize)]
pub struct Downloads {
    pub client: Download,
}

#[derive(Debug, Deserialize)]
pub struct Download {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Library {
    pub name: String,
    pub downloads: Option<LibraryDownloads>,
}

#[derive(Debug, Deserialize)]
pub struct LibraryDownloads {
    pub artifact: Option<LibraryArtifact>,
}

#[derive(Debug, Deserialize)]
pub struct LibraryArtifact {
    pub path: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
    Release,
    Snapshot,
    OldBeta,
    OldAlpha,
}