use crate::{tab::TabId, view::ViewRequest};

#[derive(Debug, Clone)]
pub enum TabAction {
    Open(ViewRequest),
    Close(TabId),
    Focus(TabId),
}
