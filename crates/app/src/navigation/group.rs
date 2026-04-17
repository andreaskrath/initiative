use crate::navigation::NavigationItem;
use crate::navigation::NavigationMessage;
use components::icon::IconName;
use style::button::ButtonClass;
use style::svg::SvgClass;
use widgets::Element;

use iced::Alignment;
use iced::Length::Fill;
use iced::widget;
use iced::widget::column;
use iced::widget::row;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavigationGroupId(u64);

pub(super) struct NavigationGroup {
    id: NavigationGroupId,
    label: String,
    items: Box<[NavigationItem]>,
    expanded: bool,
}

impl NavigationGroup {
    fn unique() -> NavigationGroupId {
        NavigationGroupId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }

    pub(super) fn new(
        label: impl Into<String>,
        items: impl IntoIterator<Item = NavigationItem>,
    ) -> Self {
        let items = items
            .into_iter()
            .collect::<Vec<NavigationItem>>()
            .into_boxed_slice();

        Self {
            id: Self::unique(),
            label: label.into().to_uppercase(),
            items,
            expanded: false,
        }
    }

    pub fn id(&self) -> NavigationGroupId {
        self.id
    }

    pub fn toggle_expansion(&mut self) {
        self.expanded = !self.expanded;
    }

    pub fn view(&self) -> Element<'_, NavigationMessage> {
        let label = components::text::muted_heading(&self.label);

        let space = widget::space::horizontal().width(Fill);

        let icon = if self.expanded {
            components::icon(IconName::ChevronDown)
        } else {
            components::icon(IconName::ChevronRight)
        }
        .class(SvgClass::Normal);

        let group_label = row![label, space, icon]
            .align_y(Alignment::Center)
            .width(Fill);

        let group_header = widget::button(group_label)
            .class(ButtonClass::Ghost)
            .on_press(NavigationMessage::ToggleGroup(self.id));

        let mut group = column![group_header].spacing(5);

        if self.expanded {
            for item in self.items.iter() {
                group = group.push(item.view());
            }
        }

        group.into()
    }
}
