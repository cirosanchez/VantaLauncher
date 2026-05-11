use slint::ComponentHandle;
use crate::config::Settings;
use crate::Theme;

pub struct ThemeManager;

impl ThemeManager {
    pub fn apply(theme: Theme, settings: &Settings) {
        theme.set_accent_1(
            Self::hex_to_color(&settings.theme_accent_1)
        );

        theme.set_accent_2(
            Self::hex_to_color(&settings.theme_accent_2)
        );

        theme.set_accent_3(
            Self::hex_to_color(&settings.theme_accent_3)
        );

        theme.set_accent_4(
            Self::hex_to_color(&settings.theme_accent_4)
        );

        theme.set_accent_5(
            Self::hex_to_color(&settings.theme_accent_5)
        );

        theme.set_accent_6(
            Self::hex_to_color(&settings.theme_accent_6)
        );

        theme.set_accent_7(
            Self::hex_to_color(&settings.theme_accent_7)
        );

        theme.set_accent_8(
            Self::hex_to_color(&settings.theme_accent_8)
        );

        theme.set_accent_9(
            Self::hex_to_color(&settings.theme_accent_9)
        );

        theme.set_accent_10(
            Self::hex_to_color(&settings.theme_accent_10)
        );

        theme.set_accent_11(
            Self::hex_to_color(&settings.theme_accent_11)
        );
    }


    fn hex_to_color(hex: &str) -> slint::Color {
        let hex = hex.trim_start_matches('#');

        let r = u8::from_str_radix(&hex[0..2], 16)
            .unwrap_or(0);

        let g = u8::from_str_radix(&hex[2..4], 16)
            .unwrap_or(0);

        let b = u8::from_str_radix(&hex[4..6], 16)
            .unwrap_or(0);

        slint::Color::from_rgb_u8(r, g, b)
    }
}