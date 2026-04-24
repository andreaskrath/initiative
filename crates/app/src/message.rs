use crate::navigation::message::NavigationMessage;
use crate::tab::TabManagerMessage;

#[derive(Debug, Clone)]
pub enum Message {
    Navigation(NavigationMessage),

    TabManager(TabManagerMessage),
}
