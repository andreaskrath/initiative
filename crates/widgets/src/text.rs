use iced::{
    Alignment, Element, Fill,
    widget::{self, Text, row, text::IntoFragment},
};

pub fn body<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .font(fonts::body::regular())
        .style(style::text::default)
}

pub fn heading<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .size(24)
        .font(fonts::heading::regular())
        .style(style::text::primary::default)
}

pub fn muted_heading<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .size(12)
        .font(fonts::heading::bold())
        .style(style::text::muted)
}

pub fn view_title<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .size(72)
        .font(fonts::heading::regular())
        .style(style::text::default)
}

pub fn view_sub_title<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .size(24)
        .font(fonts::heading::italic())
        .style(style::text::muted)
}

pub fn display<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text)
        .font(fonts::display::regular())
        .size(fonts::display::DEFAULT_DISPLAY_TEXT_SIZE)
        .center()
}

pub mod heading {
    use super::*;

    pub fn default<'a, Message: 'a>(text: impl IntoFragment<'a>) -> Element<'a, Message> {
        let heading = widget::text(text)
            .size(24)
            .font(fonts::heading::regular())
            .style(style::text::primary::default);

        let space = widget::space::horizontal().width(10);

        let line = widget::container(widget::space::horizontal())
            .height(2)
            .width(Fill)
            .style(style::container::primary::gradient_left_to_right);

        row![heading, space, line].align_y(Alignment::Center).into()
    }
}
