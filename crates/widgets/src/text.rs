use iced::widget::{
    self, Text,
    text::{IntoFragment, Shaping},
};

pub fn paragraph<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .font(fonts::body::regular())
        .style(style::text::default)
}

pub fn heading<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .size(14)
        .font(fonts::heading::regular())
        .shaping(Shaping::Advanced)
        .style(style::text::default)
}

pub fn muted_heading<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .size(12)
        .font(fonts::heading::regular())
        .shaping(Shaping::Advanced)
        .style(style::text::muted)
}

pub fn view_title<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .size(24)
        .font(fonts::heading::bold())
        .style(style::text::default)
}
