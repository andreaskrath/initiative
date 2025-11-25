use components::Icon;

use crate::tab::TabRequest;

use super::message::ExpandableNavigationItemId;

pub(super) enum NavigationItemKind {
    Expandable {
        id: ExpandableNavigationItemId,
        children: Box<[NavigationItem]>,
    },
    Navigable {
        target: TabRequest,
        suffix: Option<Icon>,
    },
}

pub(super) struct NavigationItem {
    pub(super) label: String,
    pub(super) prefix: Option<Icon>,
    pub(super) kind: NavigationItemKind,
}
