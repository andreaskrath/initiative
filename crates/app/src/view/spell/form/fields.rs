use components::multi_text_field::MultiTextFieldRule;
use components::multi_text_field::MultiTextFieldState;
use components::number_field::NumberFieldRule;
use components::number_field::NumberFieldState;
use components::select_field::SelectFieldState;
use components::text_area_field::TextAreaFieldRule;
use components::text_area_field::TextAreaFieldState;
use components::text_field::TextFieldRule;
use components::text_field::TextFieldState;
use types::Class;
use types::MagicSchool;
use types::SPELLCASTING_CLASSES;
use types::SpellArea;
use types::SpellCastingTime;
use types::SpellDuration;
use types::SpellLevel;
use types::SpellRange;
use types::SpellShapeKind;

use strum::VariantArray;

pub struct SpellFormFields {
    pub name: TextFieldState,
    pub school: SelectFieldState<MagicSchool>,
    pub level: SelectFieldState<SpellLevel>,
    pub classes: Vec<Class>,
    pub tags: MultiTextFieldState,
    pub casting_time: SelectFieldState<SpellCastingTime>,
    pub ritual: bool,
    pub concentration: bool,
    pub verbal: bool,
    pub somatic: bool,
    pub material: bool,
    pub materials: Vec<SpellMaterialInput>,
    pub duration: SelectFieldState<SpellDuration>,
    pub range: SelectFieldState<SpellRange>,
    pub area: SelectFieldState<SpellArea>,
    pub shape_kind: SelectFieldState<SpellShapeKind>,
    pub shape: SpellShapeInput,
    pub description: TextAreaFieldState,
    pub at_higher_levels: TextAreaFieldState,
    pub quote_text: TextAreaFieldState,
    pub quote_source: TextFieldState,
}

impl Default for SpellFormFields {
    fn default() -> Self {
        Self {
            name: TextFieldState::default()
                .rules([TextFieldRule::Required, TextFieldRule::Max(50)]),
            school: SelectFieldState::new(MagicSchool::VARIANTS.iter().copied(), None)
                .required(true),
            level: SelectFieldState::new(SpellLevel::VARIANTS.iter().copied(), None).required(true),
            classes: Vec::with_capacity(SPELLCASTING_CLASSES.len()),
            tags: MultiTextFieldState::default()
                .normalize(true)
                .rules([MultiTextFieldRule::Unique, MultiTextFieldRule::Min(1)]),
            casting_time: SelectFieldState::new(SpellCastingTime::VARIANTS.iter().copied(), None)
                .required(true),
            ritual: false,
            concentration: false,
            verbal: false,
            somatic: false,
            material: false,
            materials: Vec::new(),
            duration: SelectFieldState::new(SpellDuration::VARIANTS.iter().copied(), None)
                .required(true),
            range: SelectFieldState::new(SpellRange::VARIANTS.iter().copied(), None).required(true),
            area: SelectFieldState::new(SpellArea::VARIANTS.iter().copied(), None).required(true),
            shape_kind: SelectFieldState::new(SpellShapeKind::VARIANTS.iter().copied(), None)
                .required(true),
            shape: SpellShapeInput::NoShape,
            description: TextAreaFieldState::default().rules([TextAreaFieldRule::Required]),
            at_higher_levels: TextAreaFieldState::default(),
            quote_text: TextAreaFieldState::default().rules([TextAreaFieldRule::Max(1000)]),
            quote_source: TextFieldState::default().rules([TextFieldRule::Max(100)]),
        }
    }
}

#[derive(Debug, Default)]
pub struct SpellMaterialInput {
    pub material: TextFieldState,
    pub worth: TextFieldState,
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
        length: NumberFieldState,
    },
    Cube {
        length: NumberFieldState,
    },
    Cylinder {
        radius: NumberFieldState,
        height: NumberFieldState,
    },
    Line {
        width: NumberFieldState,
        length: NumberFieldState,
    },
    Sphere {
        radius: NumberFieldState,
    },
}

impl From<SpellShapeKind> for SpellShapeInput {
    fn from(kind: SpellShapeKind) -> Self {
        let input =
            NumberFieldState::new(None).rules([NumberFieldRule::Required, NumberFieldRule::Min(0)]);
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
