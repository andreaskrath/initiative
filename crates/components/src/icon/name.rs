#[derive(Debug, Clone, Copy)]
pub enum IconName {
    ChevronDown,
    ChevronRight,
    Error,
    Monster,
    NavigationClose,
    NavigationOpen,
    Plus,
    Spell,
}

impl IconName {
    pub(super) fn path(&self) -> &'static str {
        match self {
            IconName::Error => "icons/error.svg",
            IconName::ChevronDown => "icons/chevron-down.svg",
            IconName::ChevronRight => "icons/chevron-right.svg",
            IconName::Monster => "icons/monster.svg",
            IconName::NavigationClose => "icons/navigation-close.svg",
            IconName::NavigationOpen => "icons/navigation-open.svg",
            IconName::Plus => "icons/plus.svg",
            IconName::Spell => "icons/spell.svg",
        }
    }
}
