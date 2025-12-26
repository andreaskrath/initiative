use crate::{
    navigation::NavigationMessage,
    tab::{TabMessage, TabRequest},
};

#[derive(Debug, Clone)]
pub enum Message {
    Navigation(NavigationMessage),

    TabRequest(TabRequest),

    TabMessage(TabMessage),

    ThemeChanged,
}
