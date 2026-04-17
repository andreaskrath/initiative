use crate::Element;
use style::container::ContainerClass;
use style::text::TextClass;

use iced::Length;
use iced::Padding;
use iced::widget;
use iced::widget::column;

/// The proportion the header will fill of the horizontal view.
pub const HEADER_PROPORTION: u16 = 1;

/// The proportion the form will fill of the horizontal view.
pub const FORM_PROPORTION: u16 = 3;

/// Spacing between sections.
pub const SECTION_SPACING: u32 = 50;

/// Spacing internal to section bodies.
pub const BODY_SPACING: u32 = 10;

/// Spacing in between the label and the item it is labeling.
pub const LABEL_SPACING: u32 = 5;

/// Padding for all "input"-like elements; essentially anything that is part of a form.
pub const INPUT_PADDING: Padding = Padding {
    top: 8.0,
    right: 12.0,
    bottom: 8.0,
    left: 12.0,
};

/// A header for a form section.
pub fn section_header<'a, M: 'a>(title: &'a str, description: &'a str) -> Element<'a, M> {
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
pub fn section_body<'a, M: 'a>(element: impl Into<Element<'a, M>>) -> Element<'a, M> {
    widget::container(element)
        .class(ContainerClass::Surface)
        .width(Length::FillPortion(FORM_PROPORTION))
        .padding(10)
        .into()
}
