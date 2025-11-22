use strum::{Display, VariantArray};

#[derive(Display, VariantArray, Clone, Copy)]
pub enum SpellCastingTime {
    #[strum(to_string = "1 action")]
    Action,
    #[strum(to_string = "1 bonus action")]
    BonusAction,
    #[strum(to_string = "1 reaction")]
    Reaction,
    #[strum(to_string = "1 minute")]
    OneMinute,
    #[strum(to_string = "1 hour")]
    OneHour,
    #[strum(to_string = "8 hours")]
    EightHours,
    #[strum(to_string = "1 day")]
    OneDay,
}
