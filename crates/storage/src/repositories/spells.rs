use crate::Error;
use crate::models::spell::NewSpell;

#[async_trait::async_trait]
pub trait Spells {
    async fn create(&self, new_spell: NewSpell) -> Result<(), Error>;
}

pub trait SpellsRepository {
    fn spells(&self) -> &dyn Spells;
}
