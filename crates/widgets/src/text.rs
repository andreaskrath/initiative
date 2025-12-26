use iced::widget::{
    self, Text,
    text::{IntoFragment, Shaping},
};
use style::Typography;

pub fn paragraph<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
}

pub fn muted_heading<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .size(12)
        .font(Typography::heading_bold())
        .shaping(Shaping::Advanced)
        .style(style::text::muted)
}
