use crate::{navigation::group::NavigationGroupId, view::ViewRequest};

#[derive(Debug, Clone)]
pub enum NavigationMessage {
    ToggleCollapse,
    AnimationTick,
    ToggleGroup(NavigationGroupId),
    Navigate(ViewRequest),
}
