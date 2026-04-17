use crate::view::DashboardMessage;
use crate::view::SpellFormMessage;
use crate::view::SpellListMessage;

#[derive(Debug, Clone)]
pub enum TabMessage {
    Dashboard(DashboardMessage),
    SpellForm(SpellFormMessage),
    SpellList(SpellListMessage),
}
