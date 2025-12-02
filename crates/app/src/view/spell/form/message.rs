use iced::widget::text_editor::Action;
use types::{MagicSchool, SpellCastingTime, SpellDuration, SpellLevel, SpellRange};

#[derive(Debug, Clone)]
pub enum SpellFormMessage {
    NameChanged(String),
    SchoolSelected(MagicSchool),
    LevelSelected(SpellLevel),
    CastingTimeSelected(SpellCastingTime),
    DurationSelected(SpellDuration),
    RangeSelected(SpellRange),
    DescriptionChanged(Action),
    AtHigherLevelsChanged(Action),
    RitualToggled(bool),
    ConcentrationToggled(bool),
    VerbalToggled(bool),
    SomaticToggled(bool),
    MaterialsChanged(String),
}
