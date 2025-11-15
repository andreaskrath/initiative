use crate::form_mode::FormMode;

#[derive(Debug, Clone, PartialEq)]
/// The dedicated views present in the application.
pub enum View {
    Index,
    SpellForm { mode: FormMode },
}
