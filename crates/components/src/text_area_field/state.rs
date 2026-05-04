use crate::REQUIRED_ERROR_STR;

use iced::widget::text_editor::Action;
use iced::widget::text_editor::Content;

#[derive(Debug, Default)]
pub struct TextAreaFieldState {
    content: Content,
    required: bool,
    error: Option<&'static str>,
}

impl TextAreaFieldState {
    pub fn new(value: String) -> Self {
        Self {
            content: Content::with_text(&value),
            required: false,
            error: None,
        }
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Perform the specified `action` in the `TextArea`.
    pub fn perform(&mut self, action: Action) {
        self.content.perform(action);
    }

    /// Get the value of the state, if validation succeds, otherwise None.
    ///
    /// This method is most useful for "final" extraction on form submit.
    pub fn try_value(&mut self) -> Option<String> {
        let value = self.content.text();
        let start = value.len() - value.trim_start().len();
        let end = value.trim_end().len();
        let trimmed = &value[start..end];

        if trimmed.is_empty() {
            if self.required {
                tracing::debug!("required field is empty");
                self.error = Some(REQUIRED_ERROR_STR);
            }

            return None;
        }

        self.error = None;
        Some(String::from(trimmed))
    }

    pub(super) fn content(&self) -> &Content {
        &self.content
    }

    pub(super) fn error(&self) -> Option<&str> {
        self.error
    }

    pub(super) fn is_required(&self) -> bool {
        self.required
    }
}
