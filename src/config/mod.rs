use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const DEFAULT_SETTINGS: &str = include_str!("../../default_settings.toml");

// a public structure, something similar to a data class in Kotlin.
// #derive basically implements methods automatically for deserialization, serialization and debug
// Similar to @Serialize or something annotations
#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub theme_accent_1: String,
    pub theme_accent_2: String,
    pub theme_accent_3: String,
    pub theme_accent_4: String,
    pub theme_accent_5: String,
    pub theme_accent_6: String,
    pub theme_accent_7: String,
    pub theme_accent_8: String,
    pub theme_accent_9: String,
    pub theme_accent_10: String,
    pub theme_accent_11: String,
}

impl Settings {
    // public function load that returns the Self type (Settings)
    pub fn load() -> Self {
        // Get the path of the settings.toml file
        let path = Self::config_path();

        // Create a settings file if it doesn't exist
        if !path.exists() {
            //
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).unwrap();
            }

            // Write the default settings onto the file
            fs::write(&path, DEFAULT_SETTINGS).unwrap();
        }

        // Read the settings.toml file
        let content = fs::read_to_string(path).expect("Failed to read settings.toml");

        // Parse it to toml and then return the struct Settings
        // like automatically convert from toml to settings
        toml::from_str::<Settings>(&content).expect("Failed to parse settings.toml")
    }

    // Get the config path as a PathBuf
    fn config_path() -> PathBuf {
        let dirs = ProjectDirs::from(
            "dev",
            "cirosanchez",
            "VantaLauncher",
        ).unwrap();

        dirs.config_dir().join("settings.toml")
    }
}