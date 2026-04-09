mod area;
mod casting_time;
mod class;
mod duration;
mod level;
mod material;
mod range;
mod school;
mod shape;

use uuid::Uuid;

pub use area::SpellArea;
pub use casting_time::SpellCastingTime;
pub use class::{Class, SPELLCASTING_CLASSES};
pub use duration::SpellDuration;
pub use level::SpellLevel;
pub use material::SpellMaterial;
pub use range::SpellRange;
pub use school::MagicSchool;
pub use shape::{SpellShape, SpellShapeKind};

pub struct Spell {
    pub id: Option<Uuid>,
    pub name: String,
    pub school: MagicSchool,
    pub level: SpellLevel,
    // pub source: Source,
    pub classes: Vec<Class>,
    pub verbal: bool,
    pub somatic: bool,
    pub material: bool,
    pub materials: Vec<SpellMaterial>,
    pub ritual: bool,
    pub concentration: bool,
    pub casting_time: SpellCastingTime,
    pub duration: SpellDuration,
    pub range: SpellRange,
    pub area: SpellArea,
    pub shape: SpellShape,
    pub description: String,
    pub at_higher_levels: Option<String>,
    pub quote_text: String,
    pub quote_source: String,
}
