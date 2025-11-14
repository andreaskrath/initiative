use gpui::{Div, Styled, div, px};
use gpui_component::Theme;

/// A horizontal ruler.
pub fn hr(theme: &Theme) -> Div {
    let color = theme.sidebar_border;

    div().h(px(1.0)).w_full().bg(color)
}
