use crate::navigation::group::NavigationGroupId;
use crate::view::request::Request;

#[derive(Debug, Clone)]
pub enum NavigationMessage {
    ToggleCollapse,
    AnimationTick,
    ToggleGroup(NavigationGroupId),
    Navigate(Request),
}

#[derive(Debug, Clone)]
pub enum NavigationEffect {
    Navigate(Request),
}
