use crate::{Label, ValidationError};
use iced::{
    Element,
    widget::{self},
};
use tracing::{debug, warn};

/// Rules used to define requirements to the validation of a [`TextInput`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextInputRule {
    /// Set to required.
    ///
    /// This means any length greater than 0.
    Required,

    /// Set the minimum length.
    ///
    /// If you are defining both a lower and upper bound, consider [`TextInputRule::Between`].
    ///
    /// Note that this rule will only apply to a zero length string if [`TextInputRule::Required`] is also specified.
    /// This allows for rules that enforce optional lengths, only if a string is defined.
    Min(usize),

    /// Set the maximum length.
    ///
    /// If you are defining both a lower and upper bound, consider [`TextInputRule::Between`].
    ///
    /// Note that this rule will only apply to a zero length string if [`TextInputRule::Required`] is also specified.
    /// This allows for rules that enforce optional lengths, only if a string is defined.
    Max(usize),

    /// Set a lower and upper bound on length.
    ///
    /// Note that this rule will only apply to a zero length string if [`TextInputRule::Required`] is also specified.
    /// This allows for rules that enforce optional lengths, only if a string is defined.
    Between(usize, usize),
}

pub struct TextInput<Message> {
    label: String,
    value: String,
    placeholder: Option<String>,
    error: Option<String>,
    rules: Option<Box<[TextInputRule]>>,
    on_input: Option<Box<dyn Fn(String) -> Message>>,
}

impl<Message> TextInput<Message> {
    pub fn new(label: impl Into<String>, value: String) -> Self {
        Self {
            label: label.into(),
            value,
            placeholder: None,
            error: None,
            rules: None,
            on_input: None,
        }
    }

    pub fn rules(mut self, rules: impl IntoIterator<Item = TextInputRule>) -> Self {
        let collected_rules = rules
            .into_iter()
            .collect::<Vec<TextInputRule>>()
            .into_boxed_slice();

        self.rules = Some(collected_rules);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'static) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    /// Set the value of `Self`.
    pub fn set(&mut self, value: String) {
        self.value = value;
    }

    /// Get the value of `Self`.
    pub fn value(&self) -> &str {
        &self.value
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
            warn!(
                "validate called on TextInput '{}' when no rules are defined",
                self.label
            );

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

        rules.iter().any(|rule| rule == &TextInputRule::Required)
    }

    fn check_rule(&self, rule: TextInputRule) -> Result<(), ValidationError> {
        // Always pass if a zero length string is not required.
        if self.value.is_empty() && !self.is_required() {
            return Ok(());
        }

        match rule {
            TextInputRule::Required => {
                if self.value.is_empty() {
                    return Err(ValidationError::Required);
                }
            }
            TextInputRule::Min(min) => {
                if self.value.len() < min {
                    return Err(ValidationError::Short(min));
                }
            }
            TextInputRule::Max(max) => {
                if self.value.len() > max {
                    return Err(ValidationError::Long(max));
                }
            }
            TextInputRule::Between(min, max) => {
                if self.value.len() < min || self.value.len() > max {
                    return Err(ValidationError::Between(min, max));
                }
            }
        }

        Ok(())
    }
}

impl<'a, Message> TextInput<Message>
where
    Message: Clone + 'a,
{
    pub fn view(&'a self) -> Element<'a, Message> {
        let label = Label::new(&self.label)
            .required(self.is_required())
            .error(self.error());

        let value = self.value();
        let placeholder = self.placeholder.as_ref().map(|p| p.as_ref()).unwrap_or("");
        let on_input = self.on_input.as_ref();

        let input = widget::text_input(placeholder, value).on_input_maybe(on_input);

        widget::column![label, input].spacing(5).into()
    }
}
