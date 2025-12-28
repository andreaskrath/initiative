use iced::{
    Alignment, Element,
    Length::Fill,
    widget::{
        self, Row, row,
        scrollable::{Direction, Scrollbar},
    },
};
use widgets::{Icon, IconName};

use crate::{
    message::Message,
    tab::{Tab, TabAction, TabId},
};

/// The height of the tab bar.
const TAB_BAR_HEIGHT: u32 = 30;

/// The width of each tab in the tab bar.
///
/// The tab bar as a whole will always fill the entire horizontal space.
const TAB_BAR_WIDTH: u32 = 200;

pub fn tab_bar(tabs: &[Tab], active: TabId) -> Element<'_, Message> {
    let mut tab_bar = Row::with_capacity(tabs.len());

    for tab in tabs {
        let title_text = widgets::heading(tab.title());
        let title = widget::container(title_text).clip(true).padding(5);

        let mut tab_element = row![title, widget::space::horizontal().width(Fill)]
            .height(TAB_BAR_HEIGHT)
            .width(TAB_BAR_WIDTH)
            .align_y(Alignment::Center);

        if !matches!(tab, Tab::Index(_)) {
            let close_icon = Icon::new(IconName::Close).style(style::icon::default);
            let close_button = widget::button(close_icon)
                .height(TAB_BAR_HEIGHT)
                .style(style::button::ghost_danger_no_edges)
                .on_press(Message::TabAction(TabAction::Close(tab.id())));

            tab_element = tab_element.push(close_button);
        }

        let mut container = widget::container(tab_element);

        if tab.id() == active {
            container = container.style(style::container::primary_muted);
        } else {
            container = container.style(style::container::background);
        }

        let ma =
            widget::mouse_area(container).on_press(Message::TabAction(TabAction::Focus(tab.id())));

        tab_bar = tab_bar.push(ma);

        let divider =
            widget::container(widget::rule::vertical(1).style(style::rule::default)).center_y(30);

        tab_bar = tab_bar.push(divider);
    }

    widget::scrollable(tab_bar)
        .direction(Direction::Horizontal(Scrollbar::hidden()))
        .into()
}
