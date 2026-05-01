use crate::view::request::Request;

#[derive(Debug, Clone)]
pub enum Message {
    OpenSpells,
    OpenNewSpell,
}

#[derive(Debug, Clone)]
pub enum Effect {
    OpenView(Request),
}
