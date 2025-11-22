use crate::{FormMode, SpellFilter};

#[derive(Debug, Clone)]
pub enum View {
    Index,
    SpellForm { mode: FormMode },
    SpellList { filter: Option<SpellFilter> },
}
