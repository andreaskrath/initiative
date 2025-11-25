use crate::view::SpellFormMessage;

#[derive(Debug, Clone)]
pub enum TabMessagePayload {
    SpellForm(SpellFormMessage),
}
