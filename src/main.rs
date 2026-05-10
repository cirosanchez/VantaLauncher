mod config;
mod theme;
mod auth;

use std::error::Error;


use config::Settings;
use crate::theme::ThemeManager;

slint::include_modules!();

// fn main() -> Result<(), Box<dyn Error>> {
//     let settings = Settings::load();
//
//     let ui = AppWindow::new()?;
//
//     ThemeManager::apply(&ui, &settings);
//
//     ui.run()?;
//
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings = Settings::load();

    if std::env::var("AUTH_SMOKE").is_ok() {
        let auth = auth::AccountManager::new();
        let account = auth.login_microsoft_interactive().await?;
        println!("Logged in as: {}", account.username);
        println!("Account id: {}", account.id);
        println!("{:?}", account);
        return Ok(());
    }

    let ui = AppWindow::new()?;
    ThemeManager::apply(&ui, &settings);
    ui.run()?;
    Ok(())
}
