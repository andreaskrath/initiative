use gpui::{
    AnyElement, App, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, StyleRefinement, Styled, Window, div, prelude::FluentBuilder, px,
};
use gpui_component::{ActiveTheme, Icon, IconName, StyledExt, h_flex, tooltip::Tooltip, v_flex};

pub fn field<T: Styled + IntoElement>(label: impl Into<SharedString>, state: T) -> Field<T> {
    Field::new(label, state)
}

#[derive(IntoElement)]
pub struct Field<T: Styled + IntoElement + 'static> {
    label: SharedString,
    state: T,
    required: bool,
    error: Option<SharedString>,
    children: Vec<AnyElement>,
    col_span: u16,
    col_start: Option<i16>,
    col_end: Option<i16>,
    style: StyleRefinement,
}

impl<T: Styled + IntoElement> Field<T> {
    fn new(label: impl Into<SharedString>, state: T) -> Self {
        Self {
            label: label.into(),
            state,
            required: false,
            error: None,
            children: Vec::new(),
            col_span: 1,
            col_start: None,
            col_end: None,
            style: StyleRefinement::default(),
        }
    }

    /// Sets required for the field.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Sets the error for the field.
    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.error = Some(error.into());
        self
    }

    /// Sets the column span for the field.
    pub fn col_span(mut self, col_span: u16) -> Self {
        self.col_span = col_span;
        self
    }

    /// Sets the column start of the field.
    pub fn col_start(mut self, col_start: i16) -> Self {
        self.col_start = Some(col_start);
        self
    }

    /// Sets the column end of the field.
    pub fn col_end(mut self, col_end: i16) -> Self {
        self.col_end = Some(col_end);
        self
    }
}

impl<T: Styled + IntoElement> Styled for Field<T> {
    #[doc = " Returns a reference to the style memory of this element."]
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<T: Styled + IntoElement> ParentElement for Field<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Styled + IntoElement + 'static> RenderOnce for Field<T> {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_error = self.error.is_some();
        let validation_error = if let Some(error) = self.error {
            Some(
                div()
                    .id(self.label.clone())
                    .absolute()
                    .top(px(5.0))
                    .right(px(5.0))
                    .child(Icon::new(IconName::Frame).text_color(cx.theme().danger))
                    .tooltip(move |window, cx| {
                        Tooltip::new(error.clone())
                            .bg(cx.theme().primary)
                            .text_color(cx.theme().primary_foreground)
                            .top(px(-55.0))
                            .right(px(100.0))
                            .cursor_context_menu()
                            .build(window, cx)
                    }),
            )
        } else {
            None
        };

        let label = h_flex()
            .relative()
            .flex_shrink_0()
            .text_sm()
            .font_medium()
            .gap_1()
            .items_center()
            .child(div().overflow_x_hidden().child(self.label.clone()))
            .when(self.required, |this| {
                this.child(div().text_color(cx.theme().danger).child("*"))
            })
            .when_some(validation_error, |this, error| this.child(error));

        let children = div()
            .w_full()
            .flex_1()
            .overflow_x_hidden()
            .child(
                self.state
                    .when(is_error, |this| this.border_color(cx.theme().danger)),
            )
            .children(self.children);

        v_flex()
            .flex_1()
            .col_span(self.col_span)
            .when_some(self.col_start, |this, start| this.col_start(start))
            .when_some(self.col_end, |this, end| this.col_end(end))
            .child(label)
            .child(children)
    }
}
