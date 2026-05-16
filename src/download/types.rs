use std::path::PathBuf;

pub struct DownloadRequest {
    pub url: String,
    pub destination: PathBuf,
}

pub enum DownloadStatus {
    Started,
    Progress {
        // # of Bytes, not bits.
        downloaded: u64,
        total: Option<u64>,
    },
    Finished,
}