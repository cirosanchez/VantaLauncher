mod config;
mod theme_manager;

use std::error::Error;

use config::Settings;
use crate::theme_manager::ThemeManager;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::load();

    let ui = AppWindow::new()?;

    ThemeManager::apply(&ui, &settings);

    ui.run()?;

    Ok(())
}

