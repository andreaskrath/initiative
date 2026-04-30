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

pub use area::Area;
pub use casting_time::CastingTime;
pub use class::Class;
pub use class::SPELLCASTING_CLASSES;
pub use duration::Duration;
pub use level::Level;
pub use material::Material;
pub use range::Range;
pub use school::School;
pub use shape::Shape;
pub use shape::ShapeKind;

pub struct Spell {
    pub id: Option<Uuid>,
    pub name: String,
    pub school: School,
    pub level: Level,
    // pub source: Source,
    pub classes: Vec<Class>,
    pub verbal: bool,
    pub somatic: bool,
    pub material: bool,
    pub materials: Vec<Material>,
    pub ritual: bool,
    pub concentration: bool,
    pub casting_time: CastingTime,
    pub duration: Duration,
    pub range: Range,
    pub area: Area,
    pub shape: Shape,
    pub description: String,
    pub at_higher_levels: Option<String>,
    pub quote_text: String,
    pub quote_source: String,
}
