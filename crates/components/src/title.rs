use gpui::{Div, ParentElement, SharedString, Styled, div};
use gpui_component::Theme;

pub fn title(title: impl Into<SharedString>, theme: &Theme) -> Div {
    let title: SharedString = title.into();

    div().text_color(theme.primary).text_2xl().child(title)
}
