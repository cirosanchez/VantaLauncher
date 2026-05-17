use vantalauncher::minecraft::versions::types::VersionType;
use vantalauncher::minecraft::versions::service::{by_type, fetch_manifest};

#[tokio::test]
async fn fetches_manifest() {
    let releases = by_type(VersionType::OldAlpha).await.unwrap();

    releases.into_iter().for_each(|v| { println!("{:?}", v.id); });
}