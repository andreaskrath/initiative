use types::FormMode;

/// A request to open a new view.
///
/// This allows the application to intercept and redirect to existing non-unique tabs.
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    SpellForm { mode: FormMode },
    SpellList,
}
