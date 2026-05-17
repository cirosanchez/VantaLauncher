use vantalauncher::minecraft::versions::service::fetch_manifest;

#[tokio::test]
async fn fetches_manifest() {
    let manifest = fetch_manifest().await.unwrap();

    println!("{:#?}", manifest);
    assert!(!manifest.versions.is_empty());
}