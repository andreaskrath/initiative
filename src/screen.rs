pub enum Screen {
    Encounter,
    NoEncounter,
}

impl Default for Screen {
    fn default() -> Self {
        Self::NoEncounter
    }
}
