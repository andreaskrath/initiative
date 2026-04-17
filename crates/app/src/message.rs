use crate::navigation::NavigationMessage;
use crate::tab::TabAction;
use crate::tab::TabId;
use crate::tab::TabMessage;
use crate::view::ViewRequest;

#[derive(Debug, Clone)]
pub enum Message {
    Navigation(NavigationMessage),

    Tab(TabId, TabMessage),

    TabAction(TabAction),

    Navigate(ViewRequest),
}
