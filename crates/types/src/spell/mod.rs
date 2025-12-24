mod area;
mod casting_time;
mod class;
mod duration;
mod level;
mod range;
mod school;
mod shape;

use uuid::Uuid;

pub use area::SpellArea;
pub use casting_time::SpellCastingTime;
pub use class::Class;
pub use duration::SpellDuration;
pub use level::SpellLevel;
pub use range::SpellRange;
pub use school::MagicSchool;
pub use shape::SpellShape;

pub struct Spell {
    pub id: Option<Uuid>,
    pub name: String,
    pub school: MagicSchool,
    pub level: SpellLevel,
    pub verbal: bool,
    pub somatic: bool,
    pub material: Option<String>,
    pub material_consumed: bool,
    pub ritual: bool,
    pub concentration: bool,
    pub casting_time: SpellCastingTime,
    pub duration: SpellDuration,
    pub range: SpellRange,
    pub area: SpellArea,
    pub shape: SpellShape,
    pub description: String,
    pub at_higher_levels: Option<String>,
    pub classes: Vec<Class>,
}
