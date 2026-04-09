use strum::AsRefStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, AsRefStr)]
pub enum ThemeVariant {
    #[default]
    Grimoire,
}
