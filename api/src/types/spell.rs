use super::{Class, MagicSchool, SpellLevel};
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
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
    pub casting_time: String,
    pub duration: String,
    pub range: String,
    pub area: String,
    pub shape: Option<String>,
    pub description: String,
    pub at_higher_levels: Option<String>,
    pub classes: Vec<Class>,
}
