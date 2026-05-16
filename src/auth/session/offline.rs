use uuid::Uuid;
use crate::auth::account::{Account, AccountType};
use crate::auth::session::{OfflineSession, Session};

// Creates an offline account type for non-paid users.
pub(crate) fn create_offline_account(username: &str) -> Account {
    let uuid = generate_offline_uuid(username);

    Account {
        id: uuid.clone(),
        username: username.to_string(),
        account_type: AccountType::Offline,
        session: Session::Offline(OfflineSession { uuid }),
    }
}

// Generates an offline deterministic UUID for the username.
fn generate_offline_uuid(username: &str) -> String {
    let offline_name = format!("OfflinePlayer:{username}");

    Uuid::new_v3(&Uuid::NAMESPACE_DNS, offline_name.as_bytes()).to_string()
}
