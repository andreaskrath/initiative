use gpui::{
    AnyElement, App, FocusHandle, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    Styled, Window, div, prelude::FluentBuilder, px,
};
use gpui_component::{StyledExt, h_flex, scroll::ScrollbarAxis, v_flex};

pub fn container(focus_handle: FocusHandle) -> Container {
    Container::new(focus_handle)
}

#[derive(IntoElement)]
pub struct Container {
    focus_handle: FocusHandle,
    horizontal: bool,
    grid: bool,
    columns: Option<u16>,
    children: Vec<AnyElement>,
}

impl Container {
    pub fn new(focus_handle: FocusHandle) -> Self {
        Self {
            focus_handle,
            horizontal: false,
            grid: false,
            columns: None,
            children: Vec::new(),
        }
    }

    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }

    pub fn grid(mut self) -> Self {
        self.grid = true;
        self
    }

    pub fn columns(mut self, columns: u16) -> Self {
        self.columns = Some(columns);
        self
    }
}

impl ParentElement for Container {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Container {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let outer = div().size_full().track_focus(&self.focus_handle);

        let layout = if self.horizontal { h_flex() } else { v_flex() }
            .w_full()
            .when(self.grid, |this| this.grid())
            .when_some(self.columns, |this, columns| this.grid_cols(columns))
            .gap(px(10.0))
            .children(self.children);

        let inner = div()
            .scrollable(ScrollbarAxis::Vertical)
            .w(px(1200.0))
            .mx_auto()
            .p_3()
            .child(layout);

        outer.child(inner)
    }
}
