use crate::{
    navigation::NavigationMessage,
    tab::{TabAction, TabId, TabMessage},
    view::ViewRequest,
};

#[derive(Debug, Clone)]
pub enum Message {
    Navigation(NavigationMessage),

    Tab(TabId, TabMessage),

    TabAction(TabAction),

    Navigate(ViewRequest),

    ThemeChanged,
}
