use iced::{
    Theme,
    widget::svg::{self, Style},
};
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
    Close,
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
            IconName::ChevronDown => "icons/chevron-down.svg",
            IconName::ChevronRight => "icons/chevron-right.svg",
            IconName::Close => "icons/close.svg",
            IconName::Error => "icons/error.svg",
            IconName::Monster => "icons/monster.svg",
            IconName::NavigationClose => "icons/navigation-close.svg",
            IconName::NavigationOpen => "icons/navigation-open.svg",
            IconName::Plus => "icons/plus.svg",
            IconName::Spell => "icons/spell.svg",
        }
    }
}

pub struct Icon {
    name: IconName,
    size: IconSize,
    style: Option<Box<dyn Fn(&Theme) -> Style>>,
}

impl Icon {
    pub fn new(name: IconName) -> Self {
        Self {
            name,
            size: IconSize::Medium,
            style: None,
        }
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    pub fn style(mut self, style: impl Fn(&Theme) -> svg::Style + 'static) -> Self {
        self.style = Some(Box::new(style));
        self
    }
}

impl<'a, Message: 'a> From<Icon> for iced::Element<'a, Message> {
    fn from(icon: Icon) -> Self {
        let handle = match assets::icons::get(icon.name.path()) {
            Ok(handle) => handle,
            Err(err) => {
                error!("{err}");
                return iced::widget::Space::new().into();
            }
        };

        let (width, height) = icon.size.wh();

        let mut svg = iced::widget::svg(handle).width(width).height(height);

        if let Some(style_fn) = icon.style {
            // TODO: Consider if the "hovered" status is relevant, probably mostly for styling, not funtionality
            svg = svg.style(move |theme, _status| style_fn(theme));
        }

        svg.into()
    }
}
