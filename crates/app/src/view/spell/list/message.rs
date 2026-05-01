use crate::view::request::Request;

#[derive(Debug, Clone)]
pub enum Message {
    OpenNewSpell,
}

#[derive(Debug, Clone)]
pub enum Effect {
    OpenView(Request),
}
