pub mod variant;

use crate::theme::variant::ThemeVariant;

use iced::{
    Color,
    theme::{Base, Mode, Palette, Style},
};

#[derive(Debug, Clone)]
pub struct Theme {
    /// The theme variant.
    pub variant: ThemeVariant,
    /// Iced requires a theme to know its `Mode`.
    pub mode: Mode,

    /// The primary accent color, used for key actions and active navigation.
    pub primary: Color,
    /// A complementary accent color, used for secondary buttons and subtle accents.
    pub secondary: Color,
    /// A third variant seldom used, only use when primary and secondary are already in use.
    pub tertiary: Color,

    /// The base layer.
    pub background: Color,
    /// A surface in the app; elevated layers, like cards, modals etc.
    pub surface: Color,
    /// For interaction areas like text fields, text areas, select elements and the like.
    pub interaction: Color,
    /// The base text color.
    pub text: Color,
    /// The text color for dimmed text.
    pub text_dimmed: Color,

    /// A semantic color indicating information is being provided.
    pub info: Color,
    /// A semantic color indicating success from an action.
    pub success: Color,
    /// A semantic color indicating a warning to the user.
    pub warning: Color,
    /// A semantic color indicating an error occured.
    pub danger: Color,
}

impl Base for Theme {
    fn default(preference: Mode) -> Self {
        ThemeVariant::default().into()
    }

    fn mode(&self) -> Mode {
        self.mode
    }

    fn base(&self) -> Style {
        Style {
            background_color: self.background,
            text_color: self.text,
        }
    }

    fn palette(&self) -> Option<Palette> {
        None
    }

    fn name(&self) -> &str {
        self.variant.as_ref()
    }
}

impl From<ThemeVariant> for Theme {
    fn from(variant: ThemeVariant) -> Self {
        match variant {
            ThemeVariant::Grimoire => Self {
                variant,
                mode: Mode::Dark,

                primary: Color::from_rgb8(189, 48, 67),
                secondary: Color::from_rgb8(255, 179, 181),
                tertiary: Color::from_rgb8(233, 195, 73),

                background: Color::from_rgb8(19, 19, 22),
                surface: Color::from_rgb8(27, 27, 30),
                // surface: Color::from_rgb8(30, 30, 30),
                interaction: Color::from_rgb8(53, 52, 56),
                text: Color::from_rgb8(227, 224, 228),
                text_dimmed: Color::from_rgb8(111, 111, 120),

                info: Color::from_rgb8(196, 155, 235),
                success: Color::from_rgb8(16, 185, 129),
                warning: Color::from_rgb8(245, 158, 11),
                danger: Color::from_rgb8(239, 68, 68),
            },
        }
    }
}
