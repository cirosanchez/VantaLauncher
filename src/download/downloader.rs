use anyhow::Result;
use futures_util::StreamExt;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::download::types::{
    DownloadStatus,
    DownloadRequest,
};

pub struct Downloader {
    client: reqwest::Client,
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn download<F>(
        &self,
        request: DownloadRequest,
        mut callback: F,
    ) -> Result<()>
    where
        F: FnMut(DownloadStatus)
    {
        callback(DownloadStatus::Started);

        if let Some(parent) = request.destination.parent() {
            fs::create_dir_all(parent).await?;
        }

        let response = self.client
            .get(&request.url)
            .send()
            .await?
            .error_for_status()?;

        let total = response.content_length();

        let mut stream = response.bytes_stream();

        let mut file = File::create(&request.destination).await?;

        let mut downloaded: u64 = 0;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;

            file.write_all(&chunk).await?;

            downloaded += chunk.len() as u64;

            callback(DownloadStatus::Progress {
                downloaded,
                total,
            });
        }

        file.flush().await?;

        callback(DownloadStatus::Finished);

        Ok(())
    }
}