use crate::Label;
use iced::{
    Element, Length,
    widget::{self, text::Wrapping, text_editor::Action},
};

mod rules;
mod state;
mod style;

pub use rules::*;
pub use state::*;

pub fn text_area<'a, Message>(
    label: &'a str,
    state: &'a TextAreaState,
    on_action: impl Fn(Action) -> Message + 'a,
) -> TextArea<'a, Message> {
    TextArea::new(label, state, on_action)
}

pub struct TextArea<'a, Message> {
    state: &'a TextAreaState,
    label: &'a str,
    placeholder: Option<&'a str>,
    on_action: Box<dyn Fn(Action) -> Message + 'a>,
    height: Length,
}

impl<'a, Message> TextArea<'a, Message> {
    pub fn new(
        label: &'a str,
        state: &'a TextAreaState,
        on_action: impl Fn(Action) -> Message + 'a,
    ) -> Self {
        Self {
            state,
            label,
            placeholder: None,
            on_action: Box::new(on_action),
            height: Length::Shrink,
        }
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<'a, Message> From<TextArea<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(widget: TextArea<'a, Message>) -> Self {
        let label = Label::new(widget.label)
            .required(widget.state.is_required())
            .error(widget.state.error());

        let placeholder = widget.placeholder.unwrap_or("");

        let style = if widget.state.error().is_some() {
            style::error
        } else {
            style::default
        };

        let text_area = widget::text_editor(widget.state.content())
            .font(fonts::display::regular())
            .size(fonts::display::DEFAULT_DISPLAY_TEXT_SIZE)
            .style(style)
            .wrapping(Wrapping::WordOrGlyph)
            // TODO: This should be a custom markdown implementation to
            // - fix performance issues
            // - fix bleeding issues on bold and italic
            //
            // I will only ever use markdown in this scenariom so it is a non-issue to make a
            // single custom implementation IMO.
            // .highlight("markdown", iced::highlighter::Theme::InspiredGitHub)
            .placeholder(placeholder)
            .on_action(widget.on_action)
            .height(widget.height);

        widget::column![label, text_area].spacing(5).into()
    }
}
