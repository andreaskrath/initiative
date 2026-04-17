use types::{
    Class, MagicSchool, SPELLCASTING_CLASSES, SpellArea, SpellCastingTime, SpellDuration,
    SpellLevel, SpellRange, SpellShapeKind,
};

use strum::VariantArray;
use widgets::{
    number_input::{NumberInputRule, NumberInputState},
    select::SelectState,
    text_area::{TextAreaRule, TextAreaState},
    text_input::{TextInputRule, TextInputState},
};

pub struct SpellFormFields {
    pub name: TextInputState,
    pub school: SelectState<MagicSchool>,
    pub level: SelectState<SpellLevel>,
    pub classes: Vec<Class>,
    pub casting_time: SelectState<SpellCastingTime>,
    pub ritual: bool,
    pub concentration: bool,
    pub verbal: bool,
    pub somatic: bool,
    pub material: bool,
    pub materials: Vec<SpellMaterialInput>,
    pub duration: SelectState<SpellDuration>,
    pub range: SelectState<SpellRange>,
    pub area: SelectState<SpellArea>,
    pub shape_kind: SelectState<SpellShapeKind>,
    pub shape: SpellShapeInput,
    pub description: TextAreaState,
    pub at_higher_levels: TextAreaState,
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
            classes: Vec::with_capacity(SPELLCASTING_CLASSES.len()),
            casting_time: SelectState::new(SpellCastingTime::VARIANTS.iter().copied(), None)
                .required(true),
            ritual: false,
            concentration: false,
            verbal: false,
            somatic: false,
            material: false,
            materials: Vec::new(),
            duration: SelectState::new(SpellDuration::VARIANTS.iter().copied(), None)
                .required(true),
            range: SelectState::new(SpellRange::VARIANTS.iter().copied(), None).required(true),
            area: SelectState::new(SpellArea::VARIANTS.iter().copied(), None).required(true),
            shape_kind: SelectState::new(SpellShapeKind::VARIANTS.iter().copied(), None)
                .required(true),
            shape: SpellShapeInput::NoShape,
            description: TextAreaState::default().rules([TextAreaRule::Required]),
            at_higher_levels: TextAreaState::default(),
            quote_text: TextAreaState::default().rules([TextAreaRule::Max(1000)]),
            quote_source: TextInputState::default().rules([TextInputRule::Max(100)]),
        }
    }
}

#[derive(Debug, Default)]
pub struct SpellMaterialInput {
    pub material: TextInputState,
    pub worth: TextInputState,
    pub consumed: bool,
}

impl SpellMaterialInput {
    /// Check if a spell material is empty.
    pub fn is_empty(&self) -> bool {
        self.material.value().trim().is_empty() && self.worth.value().trim().is_empty()
    }
}

pub enum SpellShapeInput {
    NoShape,
    Cone {
        length: NumberInputState,
    },
    Cube {
        length: NumberInputState,
    },
    Cylinder {
        radius: NumberInputState,
        height: NumberInputState,
    },
    Line {
        width: NumberInputState,
        length: NumberInputState,
    },
    Sphere {
        radius: NumberInputState,
    },
}

impl From<SpellShapeKind> for SpellShapeInput {
    fn from(kind: SpellShapeKind) -> Self {
        let input =
            NumberInputState::new(None).rules([NumberInputRule::Required, NumberInputRule::Min(0)]);
        match kind {
            SpellShapeKind::NoShape => Self::NoShape,
            SpellShapeKind::Cone => Self::Cone {
                length: input.clone(),
            },
            SpellShapeKind::Cube => Self::Cube {
                length: input.clone(),
            },
            SpellShapeKind::Cylinder => Self::Cylinder {
                radius: input.clone(),
                height: input.clone(),
            },
            SpellShapeKind::Line => Self::Line {
                width: input.clone(),
                length: input.clone(),
            },
            SpellShapeKind::Sphere => Self::Sphere {
                radius: input.clone(),
            },
        }
    }
}
