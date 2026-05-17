use vantalauncher::minecraft::versions::types::VersionType;
use vantalauncher::minecraft::versions::service::{by_type, get_version_metadata};

use anyhow::Result;

#[tokio::test]
async fn fetches_manifest() -> Result<()> {
    let versions = by_type(VersionType::Release).await?;

    let version = versions
        .iter()
        .find(|v| v.id == "1.21.5")
        .unwrap();

    let metadata = get_version_metadata(version).await?;
    
    Ok(())
}
