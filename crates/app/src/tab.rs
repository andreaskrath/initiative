use types::{FormMode, SpellFilter};

/// This enum defines "requests" for opening tabs.
///
/// The reason this is not just the [`Tab`] enum, is because iced requires that a Message type can
/// be cloned, and cloning entire states owned by [`Tab`] is not reasonable.
#[derive(Debug, Clone)]
pub enum OpenTab {
    SpellForm { mode: FormMode },
    SpellList { filter: Option<SpellFilter> },
}

/// This enum defines the actual elements that can be shown as tabs.
pub enum Tab {}
