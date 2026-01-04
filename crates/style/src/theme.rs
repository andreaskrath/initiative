use std::sync::Arc;

use iced::{
    Color, Theme,
    theme::{Custom, Palette},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeVariant {
    #[default]
    Default,
    Dark,
    Light,
}

impl ThemeVariant {
    pub const ALL: &'static [ThemeVariant] = &[
        ThemeVariant::Default,
        ThemeVariant::Dark,
        ThemeVariant::Light,
    ];

    /// Get the name for the theme.
    pub fn name(&self) -> &'static str {
        match self {
            ThemeVariant::Default => "Default",
            ThemeVariant::Dark => "Dark Stone",
            ThemeVariant::Light => "Light",
        }
    }

    /// Get the palette for the theme.
    pub fn palette(&self) -> Palette {
        match self {
            ThemeVariant::Default => parchment(),
            ThemeVariant::Dark => dark_palette(),
            ThemeVariant::Light => light_palette(),
        }
    }
}

impl From<ThemeVariant> for Theme {
    fn from(variant: ThemeVariant) -> Self {
        let palette = variant.palette();
        let custom = Custom::new(variant.name().to_string(), palette);
        Theme::Custom(Arc::new(custom))
    }
}

pub fn parchment() -> Palette {
    Palette {
        background: Color::from_rgb8(32, 30, 35), // Cool dark slate
        text: Color::from_rgb8(225, 220, 215),    // Warm off-white
        primary: Color::from_rgb8(180, 140, 95),  // Aged gold
        success: Color::from_rgb8(95, 160, 120),  // Soft green
        warning: Color::from_rgb8(210, 160, 70),  // Amber
        danger: Color::from_rgb8(190, 85, 75),    // Muted red
    }
}

/// Dark stone theme - the original dark theme
fn dark_palette() -> Palette {
    Palette {
        // Deep charcoal stone
        background: Color::from_rgb(0.11, 0.10, 0.09), // #1c1917

        // Warm off-white - readable but not harsh
        text: Color::from_rgb(0.90, 0.87, 0.83), // #e7dfd4

        // Amber/gold - the accent color
        primary: Color::from_rgb(0.72, 0.45, 0.20), // #b87333

        // Muted emerald - for active status
        success: Color::from_rgb(0.20, 0.55, 0.35), // #338c59

        // Warm amber-orange - for caution states
        warning: Color::from_rgb(0.80, 0.52, 0.15), // #cc8526

        // Deep crimson - for danger
        danger: Color::from_rgb(0.70, 0.22, 0.22), // #b33838
    }
}

/// Light theme - clean, modern light theme
fn light_palette() -> Palette {
    Palette {
        // Pure white background
        background: Color::from_rgb(1.0, 1.0, 1.0), // #ffffff

        // Dark gray text
        text: Color::from_rgb(0.15, 0.15, 0.15), // #262626

        // Blue primary
        primary: Color::from_rgb(0.2, 0.4, 0.8), // #3366cc

        // Green success
        success: Color::from_rgb(0.2, 0.7, 0.3), // #33b34d

        // Orange warning
        warning: Color::from_rgb(0.9, 0.6, 0.1), // #e6991a

        // Red danger
        danger: Color::from_rgb(0.9, 0.2, 0.2), // #e63333
    }
}
