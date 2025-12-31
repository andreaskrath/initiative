use std::sync::Arc;

use iced::{
    Color, Theme,
    theme::{Custom, Palette},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeVariant {
    #[default]
    Parchment,
    Dark,
    Light,
}

impl ThemeVariant {
    pub const ALL: &'static [ThemeVariant] = &[
        ThemeVariant::Parchment,
        ThemeVariant::Dark,
        ThemeVariant::Light,
    ];

    /// Get the name for the theme.
    pub fn name(&self) -> &'static str {
        match self {
            ThemeVariant::Parchment => "Parchment",
            ThemeVariant::Dark => "Dark Stone",
            ThemeVariant::Light => "Light",
        }
    }

    /// Get the palette for the theme.
    pub fn palette(&self) -> Palette {
        match self {
            ThemeVariant::Parchment => parchment_palette(),
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

/// Parchment theme - warm, aged manuscript feel
fn parchment_palette() -> Palette {
    Palette {
        // Aged parchment - warm cream with slight yellow tint
        background: Color::from_rgb(0.88, 0.835, 0.674), // #e2d5ac

        // Dark sepia - like old ink on parchment
        text: Color::from_rgb(0.20, 0.16, 0.12), // #33291f

        // Rich burgundy - like wax seals and illuminated manuscript accents
        primary: Color::from_rgb(0.60, 0.25, 0.25), // #994040

        // Muted sage green - like old heraldic colors
        success: Color::from_rgb(0.35, 0.50, 0.40), // #598066

        // Warm ochre - medieval gold leaf
        warning: Color::from_rgb(0.75, 0.60, 0.25), // #bf9940

        // Deep vermillion - urgent warnings in old tomes
        danger: Color::from_rgb(0.70, 0.30, 0.25), // #b34d40
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
