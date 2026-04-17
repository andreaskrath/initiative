use style::text::TextClass;
use style::theme::Theme;

use iced::widget;
use iced::widget::text::IntoFragment;

/// Type alias for [`iced::widget::Text`] to use custom Theme from the `style` crate.
pub type Text<'a> = iced::widget::Text<'a, Theme>;

/// Used for labels on input fields.
pub fn label<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .size(14)
        .class(TextClass::Secondary)
        .font(fonts::display::bold())
}

/// Used for adding small detail descriptions to certain elements.
pub fn detail<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .size(14)
        .class(TextClass::Dimmed)
        .font(fonts::display::regular())
}

pub fn body<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .font(fonts::body::regular())
        .class(TextClass::Normal)
}

pub fn heading<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .size(24)
        .font(fonts::heading::regular())
        .class(TextClass::Primary)
}

pub fn muted_heading<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .size(12)
        .font(fonts::heading::bold())
        .class(TextClass::Dimmed)
}

pub fn view_title<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .size(72)
        .font(fonts::heading::regular())
        .class(TextClass::Normal)
}

pub fn view_sub_title<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .size(24)
        .font(fonts::heading::italic())
        .class(TextClass::Dimmed)
}

pub fn display<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .font(fonts::display::regular())
        .size(fonts::display::DEFAULT_DISPLAY_TEXT_SIZE)
        .center()
}
