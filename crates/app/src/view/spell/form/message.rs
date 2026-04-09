use iced::widget::text_editor::Action;
use types::{
    Class, MagicSchool, SpellArea, SpellCastingTime, SpellDuration, SpellLevel, SpellRange,
    SpellShapeKind,
};

#[derive(Debug, Clone)]
pub enum SpellFormMessage {
    NameChanged(String),
    SchoolSelected(MagicSchool),
    LevelSelected(SpellLevel),
    ClassToggled(Class),
    CastingTimeSelected(SpellCastingTime),
    RitualToggled,
    ConcentrationToggled,
    VerbalToggled,
    SomaticToggled,
    MaterialToggled,
    MaterialChanged(usize, String),
    MaterialWorthChanged(usize, String),
    MaterialConsumed(usize),
    DurationSelected(SpellDuration),
    RangeSelected(SpellRange),
    AreaSelected(SpellArea),
    ShapeKindSelected(SpellShapeKind),
    ShapeLengthChanged(String),
    ShapeRadiusChanged(String),
    ShapeHeightChanged(String),
    ShapeWidthChanged(String),
    DescriptionChanged(Action),
    AtHigherLevelsChanged(Action),
    QuoteTextChanged(Action),
    QuoteSourceChanged(String),
}
