use strum::VariantArray;
use types::{MagicSchool, SpellCastingTime, SpellDuration, SpellLevel, SpellRange};
use widgets::{SelectState, TextAreaRule, TextAreaState, TextInputRule, TextInputState};

pub struct SpellFormFields {
    pub name: TextInputState,
    pub school: SelectState<MagicSchool>,
    pub level: SelectState<SpellLevel>,
    pub casting_time: SelectState<SpellCastingTime>,
    pub duration: SelectState<SpellDuration>,
    pub range: SelectState<SpellRange>,
    pub description: TextAreaState,
    pub at_higher_levels: TextAreaState,
    pub ritual: bool,
    pub concentration: bool,
    pub verbal: bool,
    pub somatic: bool,
    pub materials: Vec<SpellMaterialInput>,
    pub quote_text: TextAreaState,
    pub quote_source: TextInputState,
}

impl Default for SpellFormFields {
    fn default() -> Self {
        Self {
            name: TextInputState::default()
                .rules([TextInputRule::Required, TextInputRule::Max(50)]),
            school: SelectState::new(MagicSchool::VARIANTS.iter().copied(), None).required(true),
            level: SelectState::new(SpellLevel::VARIANTS.iter().copied(), None).required(true),
            casting_time: SelectState::new(SpellCastingTime::VARIANTS.iter().copied(), None)
                .required(true),
            duration: SelectState::new(SpellDuration::VARIANTS.iter().copied(), None)
                .required(true),
            range: SelectState::new(SpellRange::VARIANTS.iter().copied(), None).required(true),
            description: TextAreaState::default().rules([TextAreaRule::Required]),
            at_higher_levels: TextAreaState::default(),
            ritual: false,
            concentration: false,
            verbal: false,
            somatic: false,
            materials: Vec::new(),
            quote_text: TextAreaState::default().rules([TextAreaRule::Max(1000)]),
            quote_source: TextInputState::default().rules([TextInputRule::Max(100)]),
        }
    }
}

#[derive(Debug)]
pub struct SpellMaterialInput {
    pub label: String,
    pub material: TextInputState,
    pub consumed: bool,
}

impl SpellMaterialInput {
    pub fn new(number: usize) -> Self {
        Self {
            label: format!("Material {}", number),
            material: TextInputState::new(String::new())
                .rules([TextInputRule::Required, TextInputRule::Max(200)]),
            consumed: false,
        }
    }
}
