use crate::Error;

use sqlx::prelude::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(rename_all = "snake_case")]
pub enum Variant {
    School,
    Level,
    CastingTime,
    Duration,
    Range,
    Area,
}

#[async_trait::async_trait]
pub trait Options {
    async fn list_options(&self, variant: Variant) -> Result<Box<[String]>, Error>;
}

pub trait OptionsRepository {
    fn options(&self) -> &dyn Options;
}
