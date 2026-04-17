use crate::tab::TabId;
use crate::view::ViewRequest;

#[derive(Debug, Clone)]
pub enum TabAction {
    Open(ViewRequest),
    Close(TabId),
    Focus(TabId),
}
