pub(crate) mod microsoft;
pub(crate) mod offline;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum Session {
    Microsoft(MicrosoftSession),
    Offline(OfflineSession),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MicrosoftSession {
    pub(crate) access_token: String,
    pub(crate) refresh_token: String,
    pub(crate) uuid: String,
    pub(crate) expires_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct OfflineSession {
    pub(crate) uuid: String,
}
