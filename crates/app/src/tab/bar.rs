use crate::message::Message;
use crate::tab::Tab;
use crate::tab::TabAction;
use crate::tab::TabId;
use components::icon::IconName;
use components::text::Text;
use style::container::ContainerClass;
use style::svg::SvgClass;
use widgets::Element;

use iced::Alignment;
use iced::Length::Fill;
use iced::widget;
use iced::widget::Row;
use iced::widget::row;
use iced::widget::scrollable::Direction;
use iced::widget::scrollable::Scrollbar;
use iced::widget::text::IntoFragment;

/// The height of the tab bar.
const TAB_BAR_HEIGHT: u32 = 30;

/// The width of each tab in the tab bar.
///
/// The tab bar as a whole will always fill the entire horizontal space.
const TAB_BAR_WIDTH: u32 = 200;

pub fn tab_bar(tabs: &[(TabId, Tab)], active: TabId) -> Element<'_, Message> {
    let mut tab_bar = Row::with_capacity(tabs.len());

    for (tab_id, tab) in tabs {
        let title_text = title(tab.title());
        let title = widget::container(title_text).clip(true).padding(5);

        let mut tab_element = row![title, widget::space::horizontal().width(Fill)]
            .height(TAB_BAR_HEIGHT)
            .width(TAB_BAR_WIDTH)
            .align_y(Alignment::Center);

        if !matches!(tab, Tab::Dashboard(_)) {
            let close_icon = components::icon(IconName::Close).class(SvgClass::Normal);
            let close_button = widget::button(close_icon)
                .height(TAB_BAR_HEIGHT)
                .on_press(Message::TabAction(TabAction::Close(*tab_id)));

            tab_element = tab_element.push(close_button);
        }

        let mut container = widget::container(tab_element);

        if *tab_id == active {
            container = container.class(ContainerClass::Surface);
        } else {
            container = container.class(ContainerClass::Ghost);
        }

        let ma =
            widget::mouse_area(container).on_press(Message::TabAction(TabAction::Focus(*tab_id)));

        tab_bar = tab_bar.push(ma);

        let divider = widget::container(widget::rule::vertical(1)).center_y(30);

        tab_bar = tab_bar.push(divider);
    }

    widget::scrollable(tab_bar)
        .direction(Direction::Horizontal(Scrollbar::hidden()))
        .width(Fill)
        .into()
}

fn title<'a>(text: impl IntoFragment<'a>) -> Text<'a> {
    widget::text(text).font(fonts::display::bold())
}
