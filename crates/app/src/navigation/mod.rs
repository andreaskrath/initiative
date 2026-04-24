mod group;
mod item;
pub mod message;

use crate::navigation::group::NavigationGroup;
use crate::navigation::item::NavigationItem;
use crate::navigation::message::NavigationEffect;
use crate::navigation::message::NavigationMessage;
use crate::view::request::ViewRequest;
use components::icon::IconName;
use widgets::Element;
use widgets::animation::Animation;

use iced::Fill;
use iced::Subscription;
use iced::Task;
use iced::widget::column;
use iced::widget::container;
use iced::widget::row;
use iced::widget::rule;
use iced::widget::space;

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
    pub fn update(
        &mut self,
        message: NavigationMessage,
    ) -> (Task<NavigationMessage>, Option<NavigationEffect>) {
        match message {
            NavigationMessage::ToggleCollapse => {
                self.collapsed = !self.collapsed;

                let target = if self.collapsed {
                    NAVIGATION_WIDTH_COLLAPSED
                } else {
                    NAVIGATION_WIDTH_EXPANDED
                };

                self.collapse_animation.retarget(target);

                (Task::none(), None)
            }
            NavigationMessage::AnimationTick => {
                self.collapse_animation.tick();

                (Task::none(), None)
            }
            NavigationMessage::ToggleGroup(id) => {
                if let Some(group) = self.groups.iter_mut().find(|g| g.id() == id) {
                    group.toggle_expansion();
                }

                (Task::none(), None)
            }
            NavigationMessage::Navigate(view) => {
                (Task::none(), Some(NavigationEffect::Navigate(view)))
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

            let groups_container = container(groups).height(Fill).width(width).clip(true);

            let divider = rule::vertical(1);

            row![groups_container, divider].into()
        }
    }

    /// Whether the sidebar is currently collapsed.
    pub fn collapsed(&self) -> bool {
        self.collapsed
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
