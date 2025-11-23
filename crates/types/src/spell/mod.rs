mod area;
mod casting_time;
mod class;
mod duration;
mod filter;
mod level;
mod range;
mod school;
mod shape;

use uuid::Uuid;

pub use area::SpellArea;
pub use casting_time::SpellCastingTime;
pub use class::Class;
pub use duration::SpellDuration;
pub use filter::SpellFilter;
pub use level::SpellLevel;
pub use range::SpellRange;
pub use school::MagicSchool;
pub use shape::SpellShape;

pub struct Spell {
    pub id: Option<Uuid>,
    pub name: String,
    pub school: Option<MagicSchool>,
    pub level: Option<SpellLevel>,
    pub verbal: bool,
    pub somatic: bool,
    pub material: Option<String>,
    pub material_consumed: bool,
    pub ritual: bool,
    pub concentration: bool,
    pub casting_time: Option<SpellCastingTime>,
    pub duration: Option<SpellDuration>,
    pub range: Option<SpellRange>,
    pub area: Option<SpellArea>,
    pub shape: Option<SpellShape>,
    pub description: String,
    pub at_higher_levels: Option<String>,
    pub classes: Vec<Class>,
}

impl Default for Spell {
    fn default() -> Self {
        Self {
            id: None,
            name: String::new(),
            school: None,
            level: None,
            verbal: false,
            somatic: false,
            material: None,
            material_consumed: false,
            ritual: false,
            concentration: false,
            casting_time: None,
            duration: None,
            range: None,
            area: None,
            shape: None,
            description: String::new(),
            at_higher_levels: None,
            classes: Vec::new(),
        }
    }
}
