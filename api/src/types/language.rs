#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "language", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Language {
    Abyssal,
    Celestial,
    Common,
    DeepSpeech,
    Draconic,
    Dwarvish,
    Elvish,
    Giant,
    Gnomish,
    Goblin,
    Halfling,
    Infernal,
    Orc,
    Primordial,
    Sylvan,
    Undercommon,
}
