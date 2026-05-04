pub mod spell;

use uuid::Uuid;

pub struct NewImage {
    pub id: Uuid,
    pub bytes: Box<[u8]>,
}
