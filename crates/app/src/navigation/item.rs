use components::Icon;

use crate::tab::OpenTab;

use super::message::ExpandableNavigationItemId;

pub(super) enum NavigationItemKind {
    Expandable {
        id: ExpandableNavigationItemId,
        children: Box<[NavigationItem]>,
    },
    Navigable {
        target: OpenTab,
        suffix: Option<Icon>,
    },
}

pub(super) struct NavigationItem {
    pub(super) label: String,
    pub(super) prefix: Option<Icon>,
    pub(super) kind: NavigationItemKind,
}
