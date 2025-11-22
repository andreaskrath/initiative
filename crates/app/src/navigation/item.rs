use components::Icon;
use types::{ExpandableNavigationItemId, View};

pub(super) enum NavigationItemKind {
    Expandable {
        id: ExpandableNavigationItemId,
        children: Box<[NavigationItem]>,
    },
    Navigable {
        target: View,
        suffix: Option<Icon>,
    },
}

pub(super) struct NavigationItem {
    pub(super) label: String,
    pub(super) prefix: Option<Icon>,
    pub(super) kind: NavigationItemKind,
}
