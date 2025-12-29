mod action;
mod bar;
mod id;
mod manager;
mod message;

use crate::view::{Dashboard, SpellForm, SpellList, ViewContent};

pub use action::*;
pub use id::*;
pub use manager::*;
pub use message::*;

/// The actual elements that can be shown as tabs.
pub enum Tab {
    Dashboard(Dashboard),
    SpellForm(Box<SpellForm>),
    SpellList(Box<SpellList>),
}

impl Tab {
    fn title(&self) -> &str {
        match self {
            Tab::Dashboard(dashboard) => dashboard.title(),
            Tab::SpellForm(spell_form) => spell_form.title(),
            Tab::SpellList(spell_list) => spell_list.title(),
        }
    }
}
