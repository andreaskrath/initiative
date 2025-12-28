use crate::view::{IndexMessage, SpellFormMessage, SpellListMessage};

#[derive(Debug, Clone)]
pub enum TabMessage {
    Index(IndexMessage),
    SpellForm(SpellFormMessage),
    SpellList(SpellListMessage),
}
