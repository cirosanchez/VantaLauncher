use crate::auth::types::{Account, AccountType, Session};

impl Account {
    pub fn is_microsoft(&self) -> bool {
        matches!(self.account_type, AccountType::Microsoft)
    }

    pub fn is_offline(&self) -> bool {
        matches!(self.account_type, AccountType::Offline)
    }
}
