use iced::widget::text_editor::Action;
use types::Class;
use types::MagicSchool;
use types::SpellArea;
use types::SpellCastingTime;
use types::SpellDuration;
use types::SpellLevel;
use types::SpellRange;
use types::SpellShapeKind;

#[derive(Debug, Clone)]
pub enum SpellFormMessage {
    NameChanged(String),
    SchoolSelected(MagicSchool),
    LevelSelected(SpellLevel),
    ClassToggled(Class),
    TagChanged(String),
    TagSubmitted,
    TagRemoved(usize),
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
    FlavorTextChanged(Action),
    AttributionChanged(String),
}
