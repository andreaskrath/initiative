use crate::{navigation::group::NavigationGroupId, tab::TabRequest};

#[derive(Debug, Clone)]
pub enum NavigationMessage {
    ToggleCollapse,
    AnimationTick,
    ToggleGroup(NavigationGroupId),
    Navigate(TabRequest),
}
