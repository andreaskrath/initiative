use iced::Length;
use style::svg::SvgClass;
use widgets::Element;

use tracing::error;

pub fn icon(name: IconName) -> Icon {
    Icon::new(name)
}

#[derive(Debug, Clone)]
pub enum IconSize {
    /// 12px.
    Small,

    /// 16px.
    Medium,

    /// 20 px.
    Large,

    /// Fill the available space.
    Fill,

    /// Custom icon size.
    Custom(u32),
}

impl IconSize {
    fn dimensions(self) -> (Length, Length) {
        match self {
            IconSize::Small => (12.into(), 12.into()),
            IconSize::Medium => (16.into(), 16.into()),
            IconSize::Large => (20.into(), 20.into()),
            IconSize::Fill => (Length::Fill, Length::Fill),
            IconSize::Custom(custom) => (custom.into(), custom.into()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IconName {
    ChevronDown,
    ChevronRight,
    Clipboard,
    Close,
    Directory,
    Error,
    Info,
    Library,
    Monster,
    NavigationClose,
    NavigationOpen,
    Plus,
    Sparkle,
    Spell,
    WandSparkles,
}

impl IconName {
    fn path(&self) -> &'static str {
        match self {
            IconName::ChevronDown => "icons/chevron-down.svg",
            IconName::ChevronRight => "icons/chevron-right.svg",
            IconName::Clipboard => "icons/clipboard.svg",
            IconName::Close => "icons/close.svg",
            IconName::Directory => "icons/directory.svg",
            IconName::Error => "icons/error.svg",
            IconName::Info => "icons/info.svg",
            IconName::Library => "icons/library.svg",
            IconName::Monster => "icons/monster.svg",
            IconName::NavigationClose => "icons/navigation-close.svg",
            IconName::NavigationOpen => "icons/navigation-open.svg",
            IconName::Plus => "icons/plus.svg",
            IconName::Sparkle => "icons/sparkle.svg",
            IconName::Spell => "icons/spell.svg",
            IconName::WandSparkles => "icons/wand-sparkles.svg",
        }
    }
}

pub struct Icon {
    name: IconName,
    size: IconSize,
    class: SvgClass,
}

impl Icon {
    pub fn new(name: IconName) -> Self {
        Self {
            name,
            size: IconSize::Medium,
            class: SvgClass::Normal,
        }
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    pub fn class(mut self, class: SvgClass) -> Self {
        self.class = class;
        self
    }
}

impl<'a, Message: 'a> From<Icon> for Element<'a, Message> {
    fn from(icon: Icon) -> Self {
        let handle = match assets::icons::get(icon.name.path()) {
            Ok(handle) => handle,
            Err(err) => {
                error!("{err}");
                return iced::widget::Space::new().into();
            }
        };

        let (width, height) = icon.size.dimensions();

        let svg = iced::widget::svg(handle)
            .width(width)
            .height(height)
            .class(icon.class);

        svg.into()
    }
}
