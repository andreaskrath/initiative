use std::sync::atomic::{AtomicU64, Ordering};

use iced::{
    Alignment, Color, Element,
    Length::Fill,
    widget::{self, column, row},
};
use widgets::{Icon, IconName};

use crate::navigation::{NavigationItem, NavigationMessage};

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
            label: label.into(),
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
        let label = widget::text(&self.label).color(Color::from_rgb(0.36, 0.36, 0.36));

        let space = widget::space::horizontal().width(Fill);

        let icon = if self.expanded {
            Icon::new(IconName::ChevronDown)
        } else {
            Icon::new(IconName::ChevronRight)
        };

        let group_label = row![label, space, icon]
            .align_y(Alignment::Center)
            .width(Fill);

        let group_header = widget::button(group_label)
            .style(widget::button::text)
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
