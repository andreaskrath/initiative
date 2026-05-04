use crate::models::NewImage;
use types::Class;

use serde::Serialize;
use uuid::Uuid;

pub struct NewSpell {
    pub id: Uuid,
    pub name: String,
    pub aliases: Box<[String]>,
    pub school: String,
    pub level: String,
    pub source: Option<String>,
    pub classes: Box<[Class]>,
    pub tags: Box<[String]>,
    pub casting_time: String,
    pub ritual: bool,
    pub concentration: bool,
    pub verbal: bool,
    pub somatic: bool,
    pub material: bool,
    pub materials: Box<[NewSpellMaterial]>,
    pub duration: String,
    pub range: String,
    pub area: String,
    pub shape: NewSpellShape,
    pub description: String,
    pub at_higher_levels: Option<String>,
    pub flavor_text: Option<String>,
    pub attribution: Option<String>,
    pub images: Box<[NewImage]>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct NewSpellMaterial {
    pub material: String,
    pub worth: Option<String>,
    pub consumed: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NewSpellShape {
    NoShape,
    Cone { length: i32 },
    Cube { length: i32 },
    Cylinder { radius: i32, height: i32 },
    Line { width: i32, length: i32 },
    Sphere { radius: i32 },
}
