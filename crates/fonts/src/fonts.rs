use iced::{
    Font,
    font::{Style, Weight},
};

pub mod body {
    //! This module defines font utility functions for a `body` font.
    //!
    //! In this context, `body` means things like text paragraphs and larger coherent sections of text.
    use super::*;

    pub const DEFAULT_BODY_TEXT_SIZE: u16 = 18;

    pub fn regular() -> Font {
        Font::with_name("Bookinsanity Remake")
    }

    pub fn bold() -> Font {
        Font {
            weight: Weight::Bold,
            ..regular()
        }
    }

    pub fn italic() -> Font {
        Font {
            style: Style::Italic,
            ..regular()
        }
    }

    pub fn bold_italic() -> Font {
        Font {
            weight: Weight::Bold,
            style: Style::Italic,
            ..regular()
        }
    }
}

pub mod heading {
    //! This module defines font utility functions for a `heading` font.
    //!
    //! In this context, `heading` means section headings in structured text and titles of views.

    use super::*;

    pub fn regular() -> Font {
        Font::with_name("Mr Eaves SC Remake")
    }

    pub fn bold() -> Font {
        Font {
            weight: Weight::Bold,
            ..regular()
        }
    }

    pub fn italic() -> Font {
        Font {
            style: Style::Italic,
            ..regular()
        }
    }

    pub fn bold_italic() -> Font {
        Font {
            weight: Weight::Bold,
            style: Style::Italic,
            ..regular()
        }
    }
}

pub mod quote {
    //! This module defines font utility functions for a `quote` font.
    //!
    //! In this context, `quote` means monster manual quotes, or stylistic descriptions of items, places, and people.

    use super::*;

    pub fn regular() -> Font {
        Font::with_name("Zatanna Misdirection")
    }

    pub fn bold() -> Font {
        Font {
            weight: Weight::Bold,
            ..regular()
        }
    }

    pub fn italic() -> Font {
        Font {
            style: Style::Italic,
            ..regular()
        }
    }

    pub fn bold_italic() -> Font {
        Font {
            weight: Weight::Bold,
            style: Style::Italic,
            ..regular()
        }
    }
}

pub mod display {
    //! This module defines font utility functions for a `display` font.
    //!
    //! In this context, `display` means a less immersive font, utilizes for data representation, navigation, and user input.

    use super::*;

    pub const DEFAULT_DISPLAY_TEXT_SIZE: u32 = 18;

    pub fn regular() -> Font {
        Font::with_name("Scaly Sans Remake")
    }

    pub fn bold() -> Font {
        Font {
            weight: Weight::Bold,
            ..regular()
        }
    }

    pub fn italic() -> Font {
        Font {
            style: Style::Italic,
            ..regular()
        }
    }

    pub fn bold_italic() -> Font {
        Font {
            weight: Weight::Bold,
            style: Style::Italic,
            ..regular()
        }
    }
}
