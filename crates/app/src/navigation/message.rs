use crate::navigation::group::NavigationGroupId;
use crate::view::request::ViewRequest;

#[derive(Debug, Clone)]
pub enum NavigationMessage {
    ToggleCollapse,
    AnimationTick,
    ToggleGroup(NavigationGroupId),
    Navigate(ViewRequest),
}

#[derive(Debug, Clone)]
pub enum NavigationEffect {
    Navigate(ViewRequest),
}
