use types::FormMode;

#[derive(Debug, Clone)]
pub enum SpellListMessage {
    CreateNewSpell,
}

#[derive(Debug, Clone)]
pub enum SpellListEffect {
    OpenSpellForm { mode: FormMode },
}
