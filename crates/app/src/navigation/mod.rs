mod group;
mod item;
mod message;

use iced::{
    Element, Fill, Subscription, Task,
    widget::{column, container, row, rule, space},
};
use widgets::{Animation, IconName};

use crate::{
    message::Message,
    navigation::{group::NavigationGroup, item::NavigationItem},
    tab::TabAction,
    view::ViewRequest,
};

pub use message::NavigationMessage;

/// The width of `Navigation` when its expanded.
const NAVIGATION_WIDTH_EXPANDED: f32 = 300.0;

/// The width of `Navigation` when its collapsed.
const NAVIGATION_WIDTH_COLLAPSED: f32 = 0.0;

pub struct Navigation {
    collapsed: bool,
    collapse_animation: Animation,
    groups: Box<[NavigationGroup]>,
}

impl Default for Navigation {
    fn default() -> Self {
        Self {
            collapsed: true,
            collapse_animation: Animation::new(
                NAVIGATION_WIDTH_COLLAPSED,
                NAVIGATION_WIDTH_COLLAPSED,
            )
            .with_speed(0.1),
            groups: groups(),
        }
    }
}

impl Navigation {
    /// Whether the sidebar is currently collapsed.
    pub fn collapsed(&self) -> bool {
        self.collapsed
    }

    pub fn update(&mut self, message: NavigationMessage) -> Task<Message> {
        match message {
            NavigationMessage::ToggleCollapse => {
                self.collapsed = !self.collapsed;

                let target = if self.collapsed {
                    NAVIGATION_WIDTH_COLLAPSED
                } else {
                    NAVIGATION_WIDTH_EXPANDED
                };

                self.collapse_animation.retarget(target);

                Task::none()
            }
            NavigationMessage::AnimationTick => {
                self.collapse_animation.tick();

                Task::none()
            }
            NavigationMessage::ToggleGroup(id) => {
                if let Some(group) = self.groups.iter_mut().find(|g| g.id() == id) {
                    group.toggle_expansion();
                }

                Task::none()
            }
            NavigationMessage::Navigate(view) => {
                Task::done(Message::TabAction(TabAction::Open(view)))
            }
        }
    }

    pub fn view(&self) -> Element<'_, NavigationMessage> {
        if self.collapsed && !self.collapse_animation.in_progress() {
            space::horizontal().into()
        } else {
            let width = self.collapse_animation.current();

            let groups = column(self.groups.iter().map(|group| group.view()))
                .spacing(50)
                .padding(15);

            let groups_container = container(groups)
                .height(Fill)
                .width(width)
                .clip(true)
                .style(style::container::background::default);

            let divider = rule::vertical(1).style(style::rule::default);

            row![groups_container, divider].into()
        }
    }

    pub fn subscription(&self) -> Subscription<NavigationMessage> {
        if self.collapse_animation.in_progress() {
            iced::time::every(self.collapse_animation.tick_rate())
                .map(|_| NavigationMessage::AnimationTick)
        } else {
            Subscription::none()
        }
    }
}

fn groups() -> Box<[NavigationGroup]> {
    let reference_items = [NavigationItem::new(
        "Spells",
        IconName::Spell,
        ViewRequest::SpellList,
    )];

    let reference_group = NavigationGroup::new("reference", reference_items);

    Box::new([reference_group])
}
