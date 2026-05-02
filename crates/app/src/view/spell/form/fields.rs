use crate::view::spell::form::Loader;
use components::image_field::state::ImageFieldState;
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
use types::SPELLCASTING_CLASSES;
use types::ShapeKind;

use strum::VariantArray;

pub struct Fields {
    pub name: TextFieldState,
    pub school: SelectFieldState<String>,
    pub level: SelectFieldState<String>,
    pub source: SelectFieldState<String>,
    pub classes: Vec<Class>,
    pub tags: MultiTextFieldState,
    pub casting_time: SelectFieldState<String>,
    pub ritual: bool,
    pub concentration: bool,
    pub verbal: bool,
    pub somatic: bool,
    pub material: bool,
    pub materials: Vec<SpellMaterialInput>,
    pub duration: SelectFieldState<String>,
    pub range: SelectFieldState<String>,
    pub area: SelectFieldState<String>,
    pub shape_kind: SelectFieldState<ShapeKind>,
    pub shape: SpellShapeInput,
    pub description: TextAreaFieldState,
    pub at_higher_levels: TextAreaFieldState,
    pub flavor_text: TextAreaFieldState,
    pub attribution: TextFieldState,
    pub images: ImageFieldState,
}

impl Fields {
    pub fn from_loader(loader: &mut Loader) -> Option<Self> {
        let schools = loader.schools.take()?;
        let levels = loader.levels.take()?;
        let casting_times = loader.casting_times.take()?;
        let durations = loader.durations.take()?;
        let ranges = loader.ranges.take()?;
        let areas = loader.areas.take()?;
        let sources = loader.sources.take()?;

        let fields = Self {
            name: TextFieldState::default()
                .rules([TextFieldRule::Required, TextFieldRule::Max(50)]),
            school: SelectFieldState::new(schools, None).required(true),
            level: SelectFieldState::new(levels, None).required(true),
            source: SelectFieldState::new(sources, None).required(true),
            classes: Vec::with_capacity(SPELLCASTING_CLASSES.len()),
            tags: MultiTextFieldState::default()
                .normalize(true)
                .rules([MultiTextFieldRule::Unique, MultiTextFieldRule::Min(1)]),
            casting_time: SelectFieldState::new(casting_times, None).required(true),
            ritual: false,
            concentration: false,
            verbal: false,
            somatic: false,
            material: false,
            materials: Vec::new(),
            duration: SelectFieldState::new(durations, None).required(true),
            range: SelectFieldState::new(ranges, None).required(true),
            area: SelectFieldState::new(areas, None).required(true),
            shape_kind: SelectFieldState::new(ShapeKind::VARIANTS.iter().copied(), None)
                .required(true),
            shape: SpellShapeInput::NoShape,
            description: TextAreaFieldState::default().rules([TextAreaFieldRule::Required]),
            at_higher_levels: TextAreaFieldState::default(),
            flavor_text: TextAreaFieldState::default().rules([TextAreaFieldRule::Max(1000)]),
            attribution: TextFieldState::default().rules([TextFieldRule::Max(100)]),
            images: ImageFieldState::default().limit(6),
        };

        Some(fields)
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

impl From<ShapeKind> for SpellShapeInput {
    fn from(kind: ShapeKind) -> Self {
        let input =
            NumberFieldState::new(None).rules([NumberFieldRule::Required, NumberFieldRule::Min(0)]);
        match kind {
            ShapeKind::NoShape => Self::NoShape,
            ShapeKind::Cone => Self::Cone { length: input },
            ShapeKind::Cube => Self::Cube { length: input },
            ShapeKind::Cylinder => Self::Cylinder {
                radius: input.clone(),
                height: input,
            },
            ShapeKind::Line => Self::Line {
                width: input.clone(),
                length: input,
            },
            ShapeKind::Sphere => Self::Sphere { radius: input },
        }
    }
}
