use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    Microsoft,
    Offline
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Session {
    Microsoft(MicrosoftSession),
    Offline(OfflineSession),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub account_type: AccountType,
    pub session: Session
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrosoftSession {
    pub access_token: String,
    pub refresh_token: String,
    pub uuid: String,
    pub expires_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineSession {
    pub uuid: String,
}