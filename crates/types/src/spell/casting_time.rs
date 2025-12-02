use strum::{Display, VariantArray};

#[derive(Debug, Display, VariantArray, Clone, Copy, PartialEq)]
pub enum SpellCastingTime {
    #[strum(to_string = "1 action")]
    Action,
    #[strum(to_string = "1 bonus action")]
    BonusAction,
    #[strum(to_string = "1 reaction")]
    Reaction,
    #[strum(to_string = "1 minute")]
    OneMinute,
    #[strum(to_string = "10 minutes")]
    TenMinutes,
    #[strum(to_string = "1 hour")]
    OneHour,
    #[strum(to_string = "8 hours")]
    EightHours,
    #[strum(to_string = "1 day")]
    OneDay,
}
