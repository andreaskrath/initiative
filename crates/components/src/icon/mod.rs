use assets::Assets;
use iced::{
    Color, Element,
    widget::{Space, svg, svg::Handle},
};
use tracing::error;

mod name;
mod size;

pub use name::IconName;
pub use size::IconSize;

#[derive(Debug, Clone)]
pub struct Icon {
    name: IconName,
    size: IconSize,
    color: Color,
}

impl Icon {
    pub fn new(name: IconName) -> Self {
        Self {
            name,
            size: IconSize::Medium,
            color: Color::BLACK,
        }
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl<'a, Message: 'a> From<Icon> for Element<'a, Message> {
    fn from(icon: Icon) -> Self {
        let Some(embedded_icon) = Assets::get(icon.name.path()) else {
            error!("failed to load icon '{}'", icon.name.path());
            return Space::new().into();
        };

        let handle = Handle::from_memory(embedded_icon.data);

        let (width, height) = icon.size.wh();

        svg(handle)
            .style(move |_, _| svg::Style {
                color: Some(icon.color),
            })
            .width(width)
            .height(height)
            .into()
    }
}
