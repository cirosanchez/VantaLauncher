mod auth;
mod config;
mod providers;
mod theme;

use std::error::Error;
use std::sync::Arc;

use crate::theme::ThemeManager;
use config::Settings;

slint::include_modules!();

async fn load_account_head_image(
    account: &auth::Account,
) -> Result<slint::Image, Box<dyn Error + Send + Sync>> {
    let response = reqwest::get(account.avatar_url()).await?;
    let body = response.error_for_status()?.bytes().await?;

    let temp_path = std::env::temp_dir().join(format!(
        "vanta-avatar-{}.png",
        account.avatar_url().rsplit('/').next().unwrap_or("0")
    ));
    std::fs::write(&temp_path, &body)?;

    Ok(slint::Image::load_from_path(&temp_path)?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = Settings::load();
    let account_manager = Arc::new(auth::AccountManager::new());

    if account_manager.get_active_account().await.is_none() {
        let login_ui = LoginWindow::new()?;
        ThemeManager::apply(login_ui.global::<Theme>(), &settings);
        let authenticated = account_manager
            .clone()
            .ensure_authenticated_with_login_window(login_ui)
            .await?;
        if !authenticated {
            return Ok(());
        }
    }

    let ui = AppWindow::new()?;
    ThemeManager::apply(ui.global::<Theme>(), &settings);

    if let Some(account) = account_manager.get_active_account().await {
        if let Ok(head_image) = load_account_head_image(&account).await {
            ui.set_account_head_image(head_image);
        }
    }

    ui.run()?;
    Ok(())
}
