use crate::auth::types::Account;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AuthData {
    pub accounts: Vec<Account>,
    pub active_account_id: Option<String>,
}

pub struct AuthStorage;

impl AuthStorage {
    pub fn load() -> AuthData {
        let path = Self::storage_path();
        if !path.exists() {
            return AuthData::default();
        }

        let content = fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    }

    pub fn save(data: &AuthData) {
        let path = Self::storage_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let content = serde_json::to_string_pretty(data).unwrap();
        let _ = fs::write(path, content);
    }

    fn storage_path() -> PathBuf {
        let dirs = ProjectDirs::from("dev", "cirosanchez", "VantaLauncher").unwrap();
        dirs.config_dir().join("accounts.json")
    }
}
