use iced::{
    Element,
    widget::{self, Button},
};

pub fn primary<'a, Message>(element: impl Into<Element<'a, Message>>) -> Button<'a, Message> {
    widget::button(element).style(style::button::primary::default)
}

pub fn secondary<'a, Message>(element: impl Into<Element<'a, Message>>) -> Button<'a, Message> {
    widget::button(element).style(style::button::background::default)
}

pub fn danger<'a, Message>(element: impl Into<Element<'a, Message>>) -> Button<'a, Message> {
    widget::button(element).style(style::button::danger::default)
}
