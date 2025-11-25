mod content;
mod message;
mod payload;
mod request;

use std::{
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
};

use iced::{Element, Task, widget::Space};
use tracing::error;
use types::FormMode;

use crate::{message::Message, view::SpellForm};

pub use content::*;
pub use message::*;
pub use payload::*;
pub use request::*;

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TabId(u64);

/// This enum defines the actual elements that can be shown as tabs.
pub enum Tab {
    SpellForm(SpellForm),
}

pub struct TabManager {
    tabs: HashMap<TabId, Tab>,
    active: TabId,
}

impl Default for TabManager {
    fn default() -> Self {
        let mut tabs = HashMap::with_capacity(1);
        let id = Self::unique();
        let initial_tab = Tab::SpellForm(SpellForm::new(FormMode::Create));
        tabs.insert(id, initial_tab);

        Self { tabs, active: id }
    }
}

impl TabManager {
    fn unique() -> TabId {
        TabId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }

    pub fn update(&mut self, message: TabMessage) -> Task<Message> {
        let (id, payload) = message.into_inner();
        let Some(tab) = self.tabs.get_mut(&id) else {
            error!(
                "could not find tab '{:?}', payload '{:?}' is lost",
                id, &payload
            );

            return Task::none();
        };

        match (tab, payload) {
            (Tab::SpellForm(spell_form), TabMessagePayload::SpellForm(spell_form_message)) => {
                return spell_form.update(spell_form_message);
            }
            (unknown_tab, unknown_payload) => unreachable!("should not be possible"),
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, TabMessage> {
        let Some(tab) = self.tabs.get(&self.active) else {
            error!("could not find tab '{:?}'", self.active);

            return Space::new(0, 0).into();
        };

        match tab {
            Tab::SpellForm(spell_form) => spell_form
                .view()
                .map(|m| TabMessage::new(self.active, TabMessagePayload::SpellForm(m))),
        }
    }
}
