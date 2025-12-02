mod item;
mod message;

use message::ExpandableNavigationItemId;

use iced::{
    Element, Fill, Subscription, Task,
    alignment::Vertical,
    widget::{button, column, row, rule, space, text, text::Wrapping},
};

use components::{Animation, Icon, IconSize, icon};
use strum::{EnumCount, VariantArray};
use types::{FormMode, MagicSchool, SpellFilter, SpellLevel};

use crate::{
    message::Message,
    navigation::item::{NavigationItem, NavigationItemKind},
    tab::TabRequest,
};

pub use message::NavigationMessage;

/// The number each level of depth indents items.
const INDENT_STEP: u32 = 10;

/// The width of `Navigation` when its expanded.
const NAVIGATION_WIDTH_EXPANDED: f32 = 300.0;

/// The width of `Navigation` when its collapsed.
const NAVIGATION_WIDTH_COLLAPSED: f32 = 0.0;

pub struct Navigation {
    collapsed: bool,
    collapse_animation: Animation,
    expanded: [bool; ExpandableNavigationItemId::COUNT],
    items: Vec<NavigationItem>,
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
            expanded: [false; ExpandableNavigationItemId::COUNT],
            items: Self::build_menu(),
        }
    }
}

impl<'a> Navigation {
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
            NavigationMessage::ToggleItem(id) => {
                self.toggle_item(id);

                Task::none()
            }
            NavigationMessage::Navigate(view) => Task::done(Message::TabRequest(view)),
            NavigationMessage::AnimationTick => {
                self.collapse_animation.tick();

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, NavigationMessage> {
        if self.collapsed && !self.collapse_animation.in_progress() {
            space::horizontal().into()
        } else {
            let width = self.collapse_animation.current();

            let navigation_menu =
                column(self.items.iter().map(|item| self.render_item(item, 0))).width(width);

            let divider = rule::vertical(1);

            row![navigation_menu, divider].height(Fill).into()
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

    fn is_expanded(&self, id: ExpandableNavigationItemId) -> bool {
        self.expanded[id]
    }

    fn toggle_item(&mut self, id: ExpandableNavigationItemId) {
        self.expanded[id] = !self.expanded[id];
    }

    fn render_item(
        &'a self,
        item: &'a NavigationItem,
        depth: u32,
    ) -> Element<'a, NavigationMessage> {
        let indent = space::horizontal().width(depth * INDENT_STEP);

        let prefix = match item.prefix {
            Some(prefix_icon) => icon(prefix_icon, IconSize::Medium),
            None => space::horizontal().width(16).into(),
        };

        let (suffix, on_press) = match &item.kind {
            NavigationItemKind::Expandable { id, children } => {
                let has_children = !children.is_empty();
                let is_expanded = self.is_expanded(*id);
                let suffix = match (has_children, is_expanded) {
                    (true, true) => icon(Icon::ChevronDown, IconSize::Medium),
                    (true, false) => icon(Icon::ChevronRight, IconSize::Medium),
                    (false, false) => space::horizontal().width(16).into(),
                    (false, true) => {
                        unreachable!("an item cannot be expanded and have no children")
                    }
                };

                let on_press = NavigationMessage::ToggleItem(*id);

                (suffix, on_press)
            }
            NavigationItemKind::Navigable { target, suffix } => {
                let suffix = match suffix {
                    Some(suffix_icon) => icon(*suffix_icon, IconSize::Medium),
                    None => space::horizontal().width(16).into(),
                };

                let on_press = NavigationMessage::Navigate(target.clone());

                (suffix, on_press)
            }
        };

        let navigation_label = row![
            indent,
            prefix,
            space::horizontal().width(10),
            text(&item.label).wrapping(Wrapping::None),
            space::horizontal().width(Fill),
            suffix
        ]
        .align_y(Vertical::Center);

        let navigation_item = button(navigation_label)
            .style(button::text)
            .on_press(on_press);

        let column = column![navigation_item].clip(true);

        if let NavigationItemKind::Expandable { id, children } = &item.kind
            && self.is_expanded(*id)
        {
            let child_elements: Vec<Element<NavigationMessage>> = children
                .iter()
                .map(|child| self.render_item(child, depth + 1))
                .collect();

            column.extend(child_elements).into()
        } else {
            column.into()
        }
    }

    fn build_menu() -> Vec<NavigationItem> {
        let spell_levels = SpellLevel::VARIANTS
            .iter()
            .map(|level| NavigationItem {
                label: level.to_string(),
                prefix: None,
                kind: NavigationItemKind::Navigable {
                    target: TabRequest::SpellList {
                        filter: Some(SpellFilter::Level(*level)),
                    },
                    suffix: None,
                },
            })
            .collect::<Vec<NavigationItem>>()
            .into_boxed_slice();

        let spell_schools = MagicSchool::VARIANTS
            .iter()
            .map(|school| NavigationItem {
                label: school.to_string(),
                prefix: None,
                kind: NavigationItemKind::Navigable {
                    target: TabRequest::SpellList {
                        filter: Some(SpellFilter::School(*school)),
                    },
                    suffix: None,
                },
            })
            .collect::<Vec<NavigationItem>>()
            .into_boxed_slice();

        let spells_menu = NavigationItem {
            label: String::from("Spells"),
            prefix: Some(Icon::Spell),
            kind: NavigationItemKind::Expandable {
                id: ExpandableNavigationItemId::Spell,
                children: Box::new([
                    NavigationItem {
                        label: String::from("Create new"),
                        prefix: None,
                        kind: NavigationItemKind::Navigable {
                            target: TabRequest::SpellForm {
                                mode: FormMode::Create,
                            },
                            suffix: Some(Icon::Plus),
                        },
                    },
                    NavigationItem {
                        label: String::from("Level"),
                        prefix: None,
                        kind: NavigationItemKind::Expandable {
                            id: ExpandableNavigationItemId::SpellLevel,
                            children: spell_levels,
                        },
                    },
                    NavigationItem {
                        label: String::from("School"),
                        prefix: None,
                        kind: NavigationItemKind::Expandable {
                            id: ExpandableNavigationItemId::SpellSchool,
                            children: spell_schools,
                        },
                    },
                ]),
            },
        };

        vec![spells_menu]
    }
}
