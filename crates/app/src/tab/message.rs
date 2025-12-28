use crate::view::{DashboardMessage, SpellFormMessage, SpellListMessage};

#[derive(Debug, Clone)]
pub enum TabMessage {
    Dashboard(DashboardMessage),
    SpellForm(SpellFormMessage),
    SpellList(SpellListMessage),
}
