use crate::auth::session::microsoft::MicrosoftAuth;
use crate::auth::session::offline::create_offline_account;
use crate::auth::storage::{AuthData, AuthStorage};
use crate::auth::Account;
use slint::ComponentHandle;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AccountManager {
    microsoft: MicrosoftAuth,
    data: Mutex<AuthData>,
}

impl AccountManager {
    pub fn new() -> Self {
        Self {
            microsoft: MicrosoftAuth::new(),
            data: Mutex::new(AuthStorage::load()),
        }
    }

    pub async fn add_account(&self, account: Account) {
        let mut data = self.data.lock().await;

        // If account already exists, update it, otherwise add it
        if let Some(existing) = data.accounts.iter_mut().find(|a| a.id == account.id) {
            *existing = account.clone();
        } else {
            data.accounts.push(account.clone());
        }

        data.active_account_id = Some(account.id);
        AuthStorage::save(&data);
    }

    pub async fn get_active_account(&self) -> Option<Account> {
        let data = self.data.lock().await;
        let id = data.active_account_id.as_ref()?;
        data.accounts.iter().find(|a| &a.id == id).cloned()
    }

    pub async fn login_offline(&self, username: &str) -> Account {
        let account = create_offline_account(username);
        self.add_account(account.clone()).await;
        account
    }

    pub async fn login_microsoft(&self) -> Option<Account> {
        let account = self.microsoft.login_interactive().await?;
        self.add_account(account.clone()).await;
        Some(account)
    }

    pub async fn ensure_authenticated_with_login_window(
        self: Arc<Self>,
        login_ui: crate::LoginWindow,
    ) -> Result<bool, slint::PlatformError> {
        if self.get_active_account().await.is_some() {
            return Ok(true);
        }

        let am_clone = self.clone();
        let handle = login_ui.as_weak();
        login_ui.on_login_microsoft(move || {
            let am = am_clone.clone();
            let h = handle.clone();
            tokio::spawn(async move {
                if am.login_microsoft().await.is_some() {
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(window) = h.upgrade() {
                            let _ = window.hide();
                        }
                    });
                }
            });
        });

        let am_clone = self.clone();
        let handle = login_ui.as_weak();
        login_ui.on_login_offline(move |username| {
            let am = am_clone.clone();
            let h = handle.clone();
            let username = username.to_string();
            tokio::spawn(async move {
                am.login_offline(&username).await;
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(window) = h.upgrade() {
                        let _ = window.hide();
                    }
                });
            });
        });

        login_ui.run()?;
        Ok(self.get_active_account().await.is_some())
    }
}
