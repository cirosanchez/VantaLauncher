use anyhow::Result;

use crate::minecraft::versions::types::{
    VersionInfo,
    VersionManifest,
    VersionType,
};

/// The piston-meta manifest url for fetching.
const VERSION_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

/// Fetches the manifest from mojang's page.
/// 
/// See types.rs for more details on the types used in this.
/// 
/// Returns the VersionManifest object
pub async fn fetch_manifest() -> Result<VersionManifest> {
    let response = reqwest::get(VERSION_MANIFEST_URL).await?;

    let manifest = response.json::<VersionManifest>().await?;

    Ok(manifest)
}

/// Fetches the manifest, and then returns all the versions contained in there.
/// 
/// Return the VersionInfo object for each version in the manifest.
pub async fn get_versions() -> Result<Vec<VersionInfo>> {
    let manifest = fetch_manifest().await?;

    Ok(manifest.versions)
}

/// Filters the versions by type, to get all releases, for example.
/// 
/// Return the versions that match the VersionType filter.
pub async fn by_type(version_type: VersionType) -> Result<Vec<VersionInfo>> {
    let versions = get_versions().await?;

    let filtered = versions
        .into_iter()
        .filter(|v| v.version_type == version_type)
        .collect();

    Ok(filtered)
}
