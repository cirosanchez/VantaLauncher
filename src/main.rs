mod config;
mod theme;
mod auth;

use std::error::Error;
use std::sync::Arc;

use config::Settings;
use crate::theme::ThemeManager;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = Settings::load();
    let account_manager = Arc::new(auth::AccountManager::new());

    if std::env::var("AUTH_SMOKE").is_ok() {
        println!("Currently saved accounts:");
        for acc in account_manager.list_accounts().await {
            println!(" - {} ({:?})", acc.username, acc.account_type);
        }

        if let Some(active) = account_manager.get_active_account().await {
            println!("Active account: {}", active.username);
        } else {
            println!("No active account.");
        }

        // Test offline login if requested
        if std::env::var("AUTH_OFFLINE").is_ok() {
            let username = std::env::var("AUTH_OFFLINE").unwrap_or_else(|_| "Player".to_string());
            let offline_acc = account_manager.login_offline(&username).await;
            println!("Logged in as offline: {} ({})", offline_acc.username, offline_acc.id);
        }

        // Test interactive if requested
        if std::env::var("AUTH_INTERACTIVE").is_ok() {
            let account = account_manager.login_microsoft_interactive().await?;
            println!("Logged in as: {}", account.username);
        }
        
        return Ok(());
    }

    // Show login window if no active account
    if account_manager.get_active_account().await.is_none() {
        let login_ui = LoginWindow::new()?;
        ThemeManager::apply(login_ui.global::<Theme>(), &settings);

        let am_clone = account_manager.clone();
        let handle = login_ui.as_weak();
        login_ui.on_login_microsoft(move || {
            let am = am_clone.clone();
            let h = handle.clone();
            tokio::spawn(async move {
                if let Ok(_) = am.login_microsoft_interactive().await {
                    let _ = slint::invoke_from_event_loop(move || {
                        h.unwrap().hide().unwrap();
                    });
                }
            });
        });

        let am_clone = account_manager.clone();
        let handle = login_ui.as_weak();
        login_ui.on_login_offline(move |username| {
            let am = am_clone.clone();
            let h = handle.clone();
            let username = username.to_string();
            tokio::spawn(async move {
                am.login_offline(&username).await;
                let _ = slint::invoke_from_event_loop(move || {
                    h.unwrap().hide().unwrap();
                });
            });
        });

        login_ui.run()?;
    }

    // Re-check if we have an account now
    if account_manager.get_active_account().await.is_none() {
        return Ok(()); // User closed login window without logging in
    }

    let ui = AppWindow::new()?;
    ThemeManager::apply(ui.global::<Theme>(), &settings);
    ui.run()?;
    Ok(())
}
