use crate::view::{IndexMessage, SpellFormMessage};

#[derive(Debug, Clone)]
pub enum TabMessagePayload {
    Index(IndexMessage),
    SpellForm(SpellFormMessage),
}
