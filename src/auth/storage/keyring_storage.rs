use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::error::Error;

const SERVICE_NAME: &str = "dev.cirosanchez.VantaLauncher";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MicrosoftSecrets {
    pub(crate) access_token: String,
    pub(crate) refresh_token: String,
}

fn account_entry(account_id: &str) -> Result<Entry, Box<dyn Error + Send + Sync>> {
    Ok(Entry::new(
        SERVICE_NAME,
        &format!("microsoft:{account_id}"),
    )?)
}

pub(crate) fn load_microsoft_secrets(
    account_id: &str,
) -> Result<Option<MicrosoftSecrets>, Box<dyn Error + Send + Sync>> {
    let entry = account_entry(account_id)?;
    match entry.get_password() {
        Ok(raw) => Ok(Some(serde_json::from_str::<MicrosoftSecrets>(&raw)?)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(err) => Err(err.into()),
    }
}

pub(crate) fn save_microsoft_secrets(
    account_id: &str,
    secrets: &MicrosoftSecrets,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let entry = account_entry(account_id)?;
    entry.set_password(&serde_json::to_string(secrets)?)?;
    Ok(())
}
