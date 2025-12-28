use crate::{
    navigation::NavigationMessage,
    tab::{TabAction, TabId, TabMessage, TabRequest},
};

#[derive(Debug, Clone)]
pub enum Message {
    Navigation(NavigationMessage),

    TabRequest(TabRequest),

    Tab(TabId, TabMessage),

    TabAction(TabAction),

    ThemeChanged,
}
