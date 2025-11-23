use crate::{navigation::NavigationMessage, tab::OpenTab};

#[derive(Debug, Clone)]
pub enum Message {
    Navigation(NavigationMessage),

    OpenTab(OpenTab),
}
