use assets::Assets;
use tracing::error;

#[derive(Debug, Clone)]
pub enum IconSize {
    /// 12px.
    Small,

    /// 16px.
    Medium,

    /// 20 px.
    Large,
}

impl IconSize {
    fn wh(self) -> (u32, u32) {
        match self {
            IconSize::Small => (12, 12),
            IconSize::Medium => (16, 16),
            IconSize::Large => (20, 20),
        }
    }
}

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
    fn path(&self) -> &'static str {
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

#[derive(Debug, Clone)]
pub struct Icon {
    name: IconName,
    size: IconSize,
    color: iced::Color,
}

impl Icon {
    pub fn new(name: IconName) -> Self {
        Self {
            name,
            size: IconSize::Medium,
            color: iced::Color::BLACK,
        }
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: iced::Color) -> Self {
        self.color = color;
        self
    }
}

impl<'a, Message: 'a> From<Icon> for iced::Element<'a, Message> {
    fn from(icon: Icon) -> Self {
        let Some(embedded_icon) = Assets::get(icon.name.path()) else {
            error!("failed to load icon '{}'", icon.name.path());
            return iced::widget::Space::new().into();
        };

        let handle = iced::widget::svg::Handle::from_memory(embedded_icon.data);

        let (width, height) = icon.size.wh();

        iced::widget::svg(handle)
            .style(move |_, _| iced::widget::svg::Style {
                color: Some(icon.color),
            })
            .width(width)
            .height(height)
            .into()
    }
}
