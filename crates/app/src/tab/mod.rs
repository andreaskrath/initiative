mod bar;
mod id;
mod manager;
mod message;

pub use id::*;
pub use manager::*;
pub use message::*;

use crate::view::View;
use crate::view::dashboard::Dashboard;
use crate::view::spell::form::SpellForm;
use crate::view::spell::list::SpellList;

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
