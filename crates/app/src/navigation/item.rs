use super::message::NavigationMessage;
use crate::view::ViewRequest;
use style::{button::ButtonClass, svg::SvgClass};
use widgets::{Element, icon::IconName};

use iced::{
    Alignment,
    Length::Fill,
    widget::{self, row, text::Wrapping},
};

pub(super) struct NavigationItem {
    label: String,
    icon: IconName,
    target: ViewRequest,
}

impl NavigationItem {
    pub fn new(label: impl Into<String>, icon: IconName, target: ViewRequest) -> Self {
        Self {
            label: label.into(),
            icon,
            target,
        }
    }

    pub fn view(&self) -> Element<'_, NavigationMessage> {
        let icon = widgets::icon(self.icon).class(SvgClass::Normal);

        let space = widget::space::horizontal().width(10);

        let label = widget::text(&self.label)
            .font(fonts::display::regular())
            .wrapping(Wrapping::None);

        let item_label = row![icon, space, label].align_y(Alignment::Center);

        widget::button(item_label)
            .class(ButtonClass::Ghost)
            .width(Fill)
            .on_press(NavigationMessage::Navigate(self.target.clone()))
            .into()
    }
}
