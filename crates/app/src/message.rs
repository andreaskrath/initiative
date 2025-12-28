use crate::{
    navigation::NavigationMessage,
    tab::{TabAction, TabId, TabMessage},
};

#[derive(Debug, Clone)]
pub enum Message {
    Navigation(NavigationMessage),

    Tab(TabId, TabMessage),

    TabAction(TabAction),

    ThemeChanged,
}
