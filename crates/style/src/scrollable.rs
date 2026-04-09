use crate::{
    NO_BORDER, NO_SHADOW, container::ContainerClass, disabled, hovered, pressed, theme::Theme,
};

use iced::{
    Background, Color,
    widget::scrollable::{AutoScroll, Catalog, Rail, Scroller, Status, Style},
};

impl Catalog for Theme {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {}

    fn style(&self, _class: &Self::Class<'_>, status: Status) -> Style {
        let (mut h, mut v) = (self.primary, self.primary);
        match status {
            Status::Active {
                is_horizontal_scrollbar_disabled,
                is_vertical_scrollbar_disabled,
            } => {
                if is_horizontal_scrollbar_disabled {
                    h = disabled(h);
                }

                if is_vertical_scrollbar_disabled {
                    v = disabled(v);
                }
            }
            Status::Hovered {
                is_horizontal_scrollbar_hovered,
                is_vertical_scrollbar_hovered,
                is_horizontal_scrollbar_disabled,
                is_vertical_scrollbar_disabled,
            } => {
                if is_horizontal_scrollbar_disabled {
                    h = disabled(h);
                } else if is_horizontal_scrollbar_hovered {
                    h = hovered(h);
                }

                if is_vertical_scrollbar_disabled {
                    v = disabled(v);
                } else if is_vertical_scrollbar_hovered {
                    v = hovered(v);
                }
            }
            Status::Dragged {
                is_horizontal_scrollbar_dragged,
                is_vertical_scrollbar_dragged,
                is_horizontal_scrollbar_disabled,
                is_vertical_scrollbar_disabled,
            } => {
                if is_horizontal_scrollbar_disabled {
                    h = disabled(h);
                } else if is_horizontal_scrollbar_dragged {
                    h = pressed(h);
                }

                if is_vertical_scrollbar_disabled {
                    v = disabled(v);
                } else if is_vertical_scrollbar_dragged {
                    v = pressed(v);
                }
            }
        };

        // The default container must have no background, otherwise pick_list elements have their
        // background be overwritten by whatever background the container sets, for some reason.
        let container = iced::widget::container::Catalog::style(self, &ContainerClass::Ghost);
        let horizontal_rail = Rail {
            background: None,
            border: NO_BORDER,
            scroller: Scroller {
                background: Background::Color(h),
                border: NO_BORDER,
            },
        };
        let vertical_rail = Rail {
            background: None,
            border: NO_BORDER,
            scroller: Scroller {
                background: Background::Color(v),
                border: NO_BORDER,
            },
        };
        let auto_scroll = AutoScroll {
            background: Background::Color(Color::TRANSPARENT),
            border: NO_BORDER,
            shadow: NO_SHADOW,
            icon: self.primary,
        };

        Style {
            container,
            vertical_rail,
            horizontal_rail,
            gap: None,
            auto_scroll,
        }
    }
}
