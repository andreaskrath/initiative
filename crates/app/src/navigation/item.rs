use iced::{
    Alignment, Element,
    Length::Fill,
    widget::{self, row, text::Wrapping},
};
use style::Typography;
use widgets::{Icon, IconName};

use crate::tab::TabRequest;

use super::message::NavigationMessage;

pub(super) struct NavigationItem {
    label: String,
    icon: IconName,
    target: TabRequest,
}

impl NavigationItem {
    pub fn new(label: impl Into<String>, icon: IconName, target: TabRequest) -> Self {
        Self {
            label: label.into(),
            icon,
            target,
        }
    }

    pub fn view(&self) -> Element<'_, NavigationMessage> {
        let icon = Icon::new(self.icon).style(style::icon::default);

        let space = widget::space::horizontal().width(10);

        let label = widget::text(&self.label)
            .font(Typography::body())
            .wrapping(Wrapping::None);

        let item_label = row![icon, space, label].align_y(Alignment::Center);

        widget::button(item_label)
            .style(style::button::ghost_background)
            .width(Fill)
            .on_press(NavigationMessage::Navigate(self.target.clone()))
            .into()
    }
}
