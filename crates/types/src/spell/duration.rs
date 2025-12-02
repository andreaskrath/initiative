use strum::{Display, VariantArray};

#[derive(Debug, Display, VariantArray, Clone, Copy, PartialEq)]
pub enum SpellDuration {
    Instantaneous,
    #[strum(to_string = "1 round")]
    OneRound,
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
    #[strum(to_string = "7 days")]
    SevenDays,
    Permanent,
    #[strum(to_string = "Until dispelled")]
    UntilDispelled,
}
