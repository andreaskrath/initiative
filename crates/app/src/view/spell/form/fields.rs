use crate::view::spell::form::Loader;
use components::image_field::state::ImageFieldState;
use components::multi_text_field::MultiTextFieldState;
use components::number_field::NumberFieldState;
use components::select_field::SelectFieldState;
use components::text_area_field::TextAreaFieldState;
use components::text_field::TextFieldState;
use storage::models::NewImage;
use storage::models::spell::NewSpell;
use storage::models::spell::NewSpellMaterial;
use storage::models::spell::NewSpellShape;
use types::Class;
use types::SPELLCASTING_CLASSES;
use types::ShapeKind;

use strum::VariantArray;
use uuid::Uuid;

pub struct Fields {
    pub name: TextFieldState,
    pub aliases: MultiTextFieldState,
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
            name: TextFieldState::default().required(true),
            aliases: MultiTextFieldState::default().normalize(false),
            school: SelectFieldState::new(schools, None).required(true),
            level: SelectFieldState::new(levels, None).required(true),
            source: SelectFieldState::new(sources, None).required(false),
            classes: Vec::with_capacity(SPELLCASTING_CLASSES.len()),
            tags: MultiTextFieldState::default().normalize(true),
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
            description: TextAreaFieldState::default().required(true),
            at_higher_levels: TextAreaFieldState::default(),
            flavor_text: TextAreaFieldState::default(),
            attribution: TextFieldState::default(),
            images: ImageFieldState::default(),
        };

        Some(fields)
    }

    pub fn try_build(&mut self) -> Option<NewSpell> {
        let name = self.name.try_value();
        let aliases = self.aliases.try_value();
        let school = self.school.try_value();
        let level = self.level.try_value();
        let source = self.source.try_value();
        let classes = self.classes.clone().into_boxed_slice();
        let tags = self.tags.try_value();
        let casting_time = self.casting_time.try_value();
        let materials = extract_materials(&mut self.materials);
        let material = !materials.is_empty();
        let duration = self.duration.try_value();
        let range = self.range.try_value();
        let area = self.area.try_value();
        let shape = extract_shape(&mut self.shape);
        let description = self.description.try_value();
        let at_higher_levels = self.at_higher_levels.try_value();
        let flavor_text = self.flavor_text.try_value();
        let attribution = self.attribution.try_value();
        let images = self
            .images
            .images()
            .into_iter()
            .map(|(id, bytes)| NewImage { id, bytes })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let new_spell = NewSpell {
            id: Uuid::new_v4(),
            name: name?,
            aliases: aliases?,
            school: school?,
            level: level?,
            source,
            classes,
            tags: tags?,
            casting_time: casting_time?,
            ritual: self.ritual,
            concentration: self.concentration,
            verbal: self.verbal,
            somatic: self.somatic,
            material,
            materials,
            duration: duration?,
            range: range?,
            area: area?,
            shape: shape?,
            description: description?,
            at_higher_levels,
            flavor_text,
            attribution,
            images,
        };

        Some(new_spell)
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
        let input = NumberFieldState::new(None).required(true);
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

fn extract_materials(materials: &mut [SpellMaterialInput]) -> Box<[NewSpellMaterial]> {
    // There is always at least one entry (the last) that is not valid,
    // so length - 1 instead of just length.
    //
    // Utilizing saturating subtraction in-case `materials` is empty, to avoid underflow.
    let mut new_materials = Vec::with_capacity(materials.len().saturating_sub(1));
    for material in materials {
        // Only utilize materials that have a material value defined.
        if let Some(material_value) = material.material.try_value() {
            let new_material = NewSpellMaterial {
                material: material_value,
                worth: material.worth.try_value(),
                consumed: material.consumed,
            };

            new_materials.push(new_material);
        }
    }

    new_materials.into_boxed_slice()
}

fn extract_shape(shape: &mut SpellShapeInput) -> Option<NewSpellShape> {
    match shape {
        SpellShapeInput::NoShape => Some(NewSpellShape::NoShape),
        SpellShapeInput::Cone { length } => {
            let length = length.try_value();

            length.map(|l| NewSpellShape::Cone { length: l })
        }
        SpellShapeInput::Cylinder { radius, height } => {
            let radius = radius.try_value();
            let height = height.try_value();

            radius.zip(height).map(|(r, h)| NewSpellShape::Cylinder {
                radius: r,
                height: h,
            })
        }
        SpellShapeInput::Line { width, length } => {
            let width = width.try_value();
            let length = length.try_value();

            width.zip(length).map(|(w, l)| NewSpellShape::Line {
                width: w,
                length: l,
            })
        }
        SpellShapeInput::Cube { length } => {
            let length = length.try_value();

            length.map(|l| NewSpellShape::Cube { length: l })
        }
        SpellShapeInput::Sphere { radius } => {
            let radius = radius.try_value();

            radius.map(|r| NewSpellShape::Sphere { radius: r })
        }
    }
}
