use anyhow::Result;

use crate::minecraft::versions::manifest::VersionManifest;

const VERSION_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

pub async fn fetch_manifest() -> Result<VersionManifest> {
    let response = reqwest::get(VERSION_MANIFEST_URL).await?;

    let manifest = response.json::<VersionManifest>().await?;

    Ok(manifest)
}