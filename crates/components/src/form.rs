use style::container::ContainerClass;
use style::text::TextClass;
use widgets::Element;

use iced::Length;
use iced::widget;
use iced::widget::column;

/// The proportion the header will fill of the horizontal view.
pub const HEADER_PROPORTION: u16 = 1;

/// The proportion the form will fill of the horizontal view.
pub const FORM_PROPORTION: u16 = 3;

/// A header for a form section.
pub fn section_header<'a, Message: 'a>(
    title: &'a str,
    description: &'a str,
) -> Element<'a, Message> {
    let title_text = widget::text(title)
        .size(32)
        .class(TextClass::Primary)
        .font(fonts::display::bold());
    let description_text = widget::text(description)
        .size(18)
        .class(TextClass::Dimmed)
        .font(fonts::display::regular());
    let titles = column![title_text, description_text];

    widget::container(titles)
        .width(Length::FillPortion(HEADER_PROPORTION))
        .into()
}

/// The body of the form section.
pub fn section_body<'a, Message: 'a>(
    element: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    widget::container(element)
        .class(ContainerClass::Surface)
        .width(Length::FillPortion(FORM_PROPORTION))
        .padding(10)
        .into()
}
