mod keyring_storage;

use crate::auth::storage::keyring_storage::{
    load_microsoft_secrets, save_microsoft_secrets, MicrosoftSecrets,
};
use crate::auth::account::Account;
use crate::auth::session::Session;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct AuthData {
    pub(crate) accounts: Vec<Account>,
    pub(crate) active_account_id: Option<String>,
}

pub(crate) struct AuthStorage;

impl AuthStorage {
    pub(crate) fn load() -> AuthData {
        let path = Self::storage_path();
        if !path.exists() {
            return AuthData::default();
        }

        let content = fs::read_to_string(path).unwrap_or_default();
        let mut data = serde_json::from_str::<AuthData>(&content).unwrap_or_default();
        Self::hydrate_accounts_from_keyring(&mut data);
        data
    }

    pub(crate) fn save(data: &AuthData) {
        let path = Self::storage_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let sanitized = Self::sanitized_for_disk(data);

        let content = serde_json::to_string_pretty(&sanitized).unwrap();
        let _ = fs::write(path, content);
        #[cfg(unix)]
        {
            // Restrict account/token file access to current user on Unix-like systems.
            let _ = fs::set_permissions(Self::storage_path(), fs::Permissions::from_mode(0o600));
        }
    }

    fn storage_path() -> PathBuf {
        let dirs = ProjectDirs::from("dev", "cirosanchez", "VantaLauncher").unwrap();
        dirs.data_local_dir().join("accounts.json")
    }

    fn hydrate_accounts_from_keyring(data: &mut AuthData) {
        for account in &mut data.accounts {
            if let Session::Microsoft(session) = &mut account.session {
                match load_microsoft_secrets(&account.id) {
                    Ok(Some(secrets)) => {
                        session.access_token = secrets.access_token;
                        session.refresh_token = secrets.refresh_token;
                    }
                    Ok(None) => {
                        session.access_token.clear();
                        session.refresh_token.clear();
                    }
                    Err(err) => eprintln!("failed to load microsoft secrets from keyring: {err}"),
                }
            }
        }
    }

    fn sanitized_for_disk(data: &AuthData) -> AuthData {
        let mut sanitized = data.clone();
        for account in &mut sanitized.accounts {
            if let Session::Microsoft(session) = &mut account.session {
                let secrets = MicrosoftSecrets {
                    access_token: session.access_token.clone(),
                    refresh_token: session.refresh_token.clone(),
                };

                if let Err(err) = save_microsoft_secrets(&account.id, &secrets) {
                    eprintln!("failed to save microsoft secrets to keyring: {err}");
                } else {
                    session.access_token.clear();
                    session.refresh_token.clear();
                }
            }
        }
        sanitized
    }
}
