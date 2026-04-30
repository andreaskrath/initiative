use crate::navigation::message::NavigationMessage;
use crate::tab::TabManagerMessage;
use storage::Error;
use storage::clients::local::Pool;

#[derive(Debug, Clone)]
pub enum Message {
    Load(LoadMessage),

    Navigation(NavigationMessage),

    TabManager(TabManagerMessage),
}

#[derive(Debug, Clone)]
pub enum LoadMessage {
    DatabaseConnected(Result<Pool, Error>),
}
