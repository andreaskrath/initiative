mod rules;
mod state;

pub use rules::*;
pub use state::*;

use crate::form::INPUT_PADDING;
use crate::form::LABEL_SPACING;
use crate::label::Label;
use style::text_editor::TextEditorClass;
use widgets::Element;

use iced::Length;
use iced::widget;
use iced::widget::text::Wrapping;
use iced::widget::text_editor::Action;

pub fn text_area_field<'a, Message>(
    label: &'a str,
    state: &'a TextAreaFieldState,
    on_action: impl Fn(Action) -> Message + 'a,
) -> TextAreaField<'a, Message> {
    TextAreaField::new(label, state, on_action)
}

pub struct TextAreaField<'a, Message> {
    state: &'a TextAreaFieldState,
    label: &'a str,
    placeholder: Option<&'a str>,
    on_action: Box<dyn Fn(Action) -> Message + 'a>,
    height: Length,
}

impl<'a, Message> TextAreaField<'a, Message> {
    pub fn new(
        label: &'a str,
        state: &'a TextAreaFieldState,
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

impl<'a, Message> From<TextAreaField<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(widget: TextAreaField<'a, Message>) -> Self {
        let label = Label::new(widget.label)
            .required(widget.state.is_required())
            .error(widget.state.error());

        let placeholder = widget.placeholder.unwrap_or("");

        let mut text_area = widget::text_editor(widget.state.content())
            .font(fonts::display::regular())
            .size(fonts::display::DEFAULT_DISPLAY_TEXT_SIZE)
            .wrapping(Wrapping::WordOrGlyph)
            .placeholder(placeholder)
            .on_action(widget.on_action)
            .padding(INPUT_PADDING)
            .height(widget.height);

        if widget.state.error().is_some() {
            text_area = text_area.class(TextEditorClass::Error);
        }

        widget::column![label, text_area]
            .spacing(LABEL_SPACING)
            .into()
    }
}
