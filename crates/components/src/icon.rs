use assets::Assets;
use iced::{
    Element,
    widget::{Space, svg, svg::Handle},
};
use tracing::error;

pub fn icon<M: 'static>(icon: Icon, size: IconSize) -> Element<'static, M> {
    let Some(embedded_icon) = Assets::get(icon.path()) else {
        error!("failed to load icon '{}'", icon.path());
        return Space::new(0, 0).into();
    };

    let handle = Handle::from_memory(embedded_icon.data);

    let (width, height) = size.wh();

    svg(handle).width(width).height(height).into()
}

#[derive(Debug, Clone, Copy)]
pub enum Icon {
    ChevronDown,
    ChevronRight,
    Error,
    Monster,
    NavigationClose,
    NavigationOpen,
    Plus,
    Spell,
}

impl Icon {
    fn path(&self) -> &'static str {
        match self {
            Icon::Error => "icons/error.svg",
            Icon::ChevronDown => "icons/chevron-down.svg",
            Icon::ChevronRight => "icons/chevron-right.svg",
            Icon::Monster => "icons/monster.svg",
            Icon::NavigationClose => "icons/navigation-close.svg",
            Icon::NavigationOpen => "icons/navigation-open.svg",
            Icon::Plus => "icons/plus.svg",
            Icon::Spell => "icons/spell.svg",
        }
    }
}

pub enum IconSize {
    /// 12px.
    Small,

    /// 16px.
    Medium,

    /// 20 px.
    Large,
}

impl IconSize {
    fn wh(self) -> (u16, u16) {
        match self {
            IconSize::Small => (12, 12),
            IconSize::Medium => (16, 16),
            IconSize::Large => (20, 20),
        }
    }
}
