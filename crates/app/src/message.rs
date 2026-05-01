use crate::view::ViewId;
use crate::view::ViewMessage;
use crate::view::request::Request;
use storage::Error;
use storage::clients::local::Pool;

#[derive(Debug, Clone)]
pub enum Message {
    Load(LoadMessage),
    OpenView(Request),
    CloseView(ViewId),
    FocusView(ViewId),
    ViewUpdated(ViewId, ViewMessage),
}

#[derive(Debug, Clone)]
pub enum LoadMessage {
    DatabaseConnected(Result<Pool, Error>),
}
