use crate::{Label, ValidationError};
use iced::{
    Element, Length,
    widget::{
        self,
        text::Wrapping,
        text_editor::{Action, Content},
    },
};
use tracing::debug;

/// Rules used to define requirements to the validation of a [`TextArea`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAreaRule {
    /// Set to required.
    ///
    /// This means any length greater than 0.
    Required,

    /// Set the minimum length.
    ///
    /// If you are defining both a lower and upper bound, consider [`TextAreaRule::Between`].
    ///
    /// Note that this rule will only apply to a zero length string if [`TextAreaRule::Required`] is also specified.
    /// This allows for rules that enforce optional lengths, only if a string is defined.
    Min(usize),

    /// Set the maximum length.
    ///
    /// If you are defining both a lower and upper bound, consider [`TextAreaRule::Between`].
    ///
    /// Note that this rule will only apply to a zero length string if [`TextAreaRule::Required`] is also specified.
    /// This allows for rules that enforce optional lengths, only if a string is defined.
    Max(usize),

    /// Set a lower and upper bound on length.
    ///
    /// Note that this rule will only apply to a zero length string if [`TextAreaRule::Required`] is also specified.
    /// This allows for rules that enforce optional lengths, only if a string is defined.
    Between(usize, usize),
}

pub struct TextArea<Message> {
    label: String,
    value: Content,
    on_action: Box<dyn Fn(Action) -> Message>,
    height: Length,
    placeholder: Option<String>,
    error: Option<String>,
    rules: Option<Box<[TextAreaRule]>>,
}

impl<Message> TextArea<Message> {
    pub fn new(
        label: impl Into<String>,
        value: &str,
        on_action: impl Fn(Action) -> Message + 'static,
    ) -> Self {
        let value = Content::with_text(value);

        Self {
            label: label.into(),
            value,
            on_action: Box::new(on_action),
            height: Length::Shrink,
            placeholder: None,
            error: None,
            rules: None,
        }
    }

    pub fn rules(mut self, rules: impl IntoIterator<Item = TextAreaRule>) -> Self {
        let collected_rules = rules
            .into_iter()
            .collect::<Vec<TextAreaRule>>()
            .into_boxed_slice();

        self.rules = Some(collected_rules);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Updates `Self` with the provided `action`.
    pub fn update(&mut self, action: Action) {
        self.value.perform(action);
    }

    /// Updates `Self` with the provided `action` and calls [`Self::update`] afterwards.
    ///
    /// This function will only call update, if the internal text state is modified.
    pub fn update_and_validate(&mut self, action: Action) {
        let should_update = matches!(action, Action::Edit(_));
        self.value.perform(action);

        if should_update {
            self.validate();
        }
    }

    /// Get the value of `Self`.
    pub fn value(&self) -> String {
        self.value.text()
    }

    /// Get the error of `Self`.
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// Validate the current value of `Self` against the defined rules.
    ///
    /// This methods short-circuits on the first error, and checks rules in the order they were
    /// defined during the creation of this struct.
    pub fn validate(&mut self) -> bool {
        self.error = None;

        let Some(rules) = &self.rules else {
            return true;
        };

        for rule in rules {
            if let Err(err) = self.check_rule(*rule) {
                debug!("setting validation error for '{}': {:?}", self.label, err);
                self.error = Some(err.to_string());
                return false;
            }
        }

        true
    }

    fn is_required(&self) -> bool {
        let Some(ref rules) = self.rules else {
            return false;
        };

        rules.iter().any(|rule| rule == &TextAreaRule::Required)
    }

    fn check_rule(&self, rule: TextAreaRule) -> Result<(), ValidationError> {
        // Always pass if a zero length string is not required.
        if self.value.is_empty() && !self.is_required() {
            return Ok(());
        }

        let text_area = self.value.text();

        match rule {
            TextAreaRule::Required => {
                if text_area.is_empty() {
                    return Err(ValidationError::Required);
                }
            }
            TextAreaRule::Min(min) => {
                if text_area.len() < min {
                    return Err(ValidationError::Short(min));
                }
            }
            TextAreaRule::Max(max) => {
                if text_area.len() > max {
                    return Err(ValidationError::Long(max));
                }
            }
            TextAreaRule::Between(min, max) => {
                if text_area.len() < min || text_area.len() > max {
                    return Err(ValidationError::Between(min, max));
                }
            }
        }

        Ok(())
    }
}

impl<'a, Message> TextArea<Message>
where
    Message: Clone + 'a,
{
    pub fn view(&'a self) -> Element<'a, Message> {
        let label = Label::new(&self.label)
            .required(self.is_required())
            .error(self.error());

        let placeholder = self.placeholder.as_ref().map(|p| p.as_ref()).unwrap_or("");
        let on_action = self.on_action.as_ref();

        let mut text_area = widget::text_editor(&self.value)
            .font(fonts::display::regular())
            .wrapping(Wrapping::WordOrGlyph)
            .placeholder(placeholder)
            .on_action(on_action)
            .height(self.height);

        if self.error().is_some() {
            text_area = text_area.style(style::text_area::error);
        } else {
            text_area = text_area.style(style::text_area::default);
        }

        widget::column![label, text_area].spacing(5).into()
    }
}
