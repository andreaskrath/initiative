use types::{FormMode, SpellFilter};

/// This enum defines "requests" to open a new tab.
///
/// This allows the [`TabManager`] to intercept and redirect to existing non-unique tabs.
///
/// The reason this is not just the [`Tab`] enum, is because iced requires that a Message type can
/// be cloned, and cloning entire states owned by [`Tab`] is not reasonable.
#[derive(Debug, Clone, PartialEq)]
pub enum TabRequest {
    SpellForm { mode: FormMode },
    SpellList { filter: Option<SpellFilter> },
}
