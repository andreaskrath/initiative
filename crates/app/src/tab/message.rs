use crate::tab::{TabId, TabMessagePayload};

#[derive(Debug, Clone)]
pub struct TabMessage {
    id: TabId,
    payload: TabMessagePayload,
}

impl TabMessage {
    pub fn new(id: TabId, payload: TabMessagePayload) -> Self {
        Self { id, payload }
    }

    pub fn into_inner(self) -> (TabId, TabMessagePayload) {
        (self.id, self.payload)
    }
}
