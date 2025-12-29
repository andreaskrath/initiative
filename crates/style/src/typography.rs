use std::sync::LazyLock;

use iced::font::{Font, Weight};
use parking_lot::RwLock;

static TYPOGRAPHY: LazyLock<RwLock<TypographyState>> =
    LazyLock::new(|| RwLock::new(TypographyState::default()));

struct TypographyState {
    body: Font,
    heading: Font,
}

impl Default for TypographyState {
    fn default() -> Self {
        Self {
            body: Font::DEFAULT,
            heading: Font::DEFAULT,
        }
    }
}

pub struct Typography;

impl Typography {
    /// Update fonts.
    pub fn set_fonts(body: Font, heading: Font) {
        let mut state = TYPOGRAPHY.write();
        state.body = body;
        state.heading = heading;
    }

    /// Get the current body font.
    pub fn body() -> Font {
        TYPOGRAPHY.read().body
    }

    /// Get the current body font with bold weight.
    pub fn body_bold() -> Font {
        Font {
            weight: Weight::Bold,
            ..TYPOGRAPHY.read().body
        }
    }

    /// Get the current heading font.
    pub fn heading() -> Font {
        TYPOGRAPHY.read().heading
    }

    /// Get the current heading font with bold weight.
    pub fn heading_bold() -> Font {
        Font {
            weight: Weight::Bold,
            ..TYPOGRAPHY.read().heading
        }
    }
}
