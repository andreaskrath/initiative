mod content;
mod message;
mod payload;
mod request;

use std::sync::atomic::{AtomicU64, Ordering};

use iced::{Element, Task, widget::Space};
use tracing::error;
use types::FormMode;

use crate::{
    message::Message,
    view::{Index, SpellForm},
};

pub use content::*;
pub use message::*;
pub use payload::*;
pub use request::*;

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TabId(u64);

/// This enum defines the actual elements that can be shown as tabs.
pub enum Tab {
    Index(Index),
    SpellForm(SpellForm),
}

impl Tab {
    fn id(&self) -> TabId {
        match self {
            Tab::Index(index) => index.id(),
            Tab::SpellForm(spell_form) => spell_form.id(),
        }
    }
}

pub struct TabManager {
    tabs: Vec<Tab>,
    active: TabId,
}

impl Default for TabManager {
    fn default() -> Self {
        let mut tabs = Vec::with_capacity(1);
        let id = Self::unique();
        let initial_tab = Tab::Index(Index::new(id));
        tabs.push(initial_tab);

        Self { tabs, active: id }
    }
}

impl TabManager {
    fn unique() -> TabId {
        TabId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }

    fn get(&self, id: TabId) -> Option<&Tab> {
        self.tabs.iter().find(|tab| id == tab.id())
    }

    fn get_mut(&mut self, id: TabId) -> Option<&mut Tab> {
        self.tabs.iter_mut().find(|tab| id == tab.id())
    }

    pub fn update(&mut self, message: TabMessage) -> Task<Message> {
        let (id, payload) = message.into_inner();
        let Some(tab) = self.get_mut(id) else {
            error!(
                "could not find tab '{:?}', payload '{:?}' is lost",
                id, &payload
            );

            return Task::none();
        };

        match (tab, payload) {
            (Tab::Index(index), TabMessagePayload::Index(index_message)) => {
                return index.update(index_message);
            }
            (Tab::SpellForm(spell_form), TabMessagePayload::SpellForm(spell_form_message)) => {
                return spell_form.update(spell_form_message);
            }
            (unknown_tab, unknown_payload) => {
                error!(
                    "invalid tab '{:?}' and payload '{:?}'",
                    unknown_tab.id(),
                    unknown_payload
                );
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, TabMessage> {
        let Some(tab) = self.get(self.active) else {
            error!("could not find tab '{:?}'", self.active);

            return Space::new(0, 0).into();
        };

        match tab {
            Tab::SpellForm(spell_form) => spell_form
                .view()
                .map(|m| TabMessage::new(spell_form.id(), TabMessagePayload::SpellForm(m))),
            Tab::Index(index) => index
                .view()
                .map(|m| TabMessage::new(index.id(), TabMessagePayload::Index(m))),
        }
    }

    pub fn handle_request(&mut self, request: TabRequest) -> Task<Message> {
        match request {
            TabRequest::Index => {
                // Check if index already exists
                let Some(tab) = self.tabs.iter().find(|tab| matches!(tab, Tab::Index(_))) else {
                    let id = Self::unique();
                    let new_tab = Tab::Index(Index::new(id));
                    self.tabs.push(new_tab);
                    self.active = id;

                    return Task::none();
                };

                self.active = tab.id();
            }
            TabRequest::SpellForm { mode } => todo!(),
            TabRequest::SpellList { filter } => todo!(),
        }

        Task::none()
    }
}
