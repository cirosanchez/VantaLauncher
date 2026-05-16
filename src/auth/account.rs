use crate::auth::session::Session;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum AccountType {
    Microsoft,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub(crate) id: String,
    pub(crate) username: String,
    pub(crate) account_type: AccountType,
    pub(crate) session: Session,
}

impl Account {
    pub fn avatar_url(&self) -> String {
        match self.account_type {
            AccountType::Microsoft => format!("https://mc-heads.net/avatar/{}", self.id),
            AccountType::Offline => "https://mc-heads.net/avatar/0".to_string(),
        }
    }
}
