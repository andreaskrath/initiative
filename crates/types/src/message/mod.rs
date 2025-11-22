mod navigation;

use crate::View;

pub use navigation::ExpandableNavigationItemId;
pub use navigation::NavigationMessage;

#[derive(Debug, Clone)]
pub enum Message {
    Navigation(NavigationMessage),

    OpenTab(View),
}
