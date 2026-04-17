use super::TextAreaFieldRule;
use crate::ValidationError;
use iced::widget::text_editor::Action;
use iced::widget::text_editor::Content;
use tracing::debug;

#[derive(Debug, Default)]
pub struct TextAreaFieldState {
    content: Content,
    rules: Option<Box<[TextAreaFieldRule]>>,
    error: Option<String>,
}

impl TextAreaFieldState {
    pub fn new(value: String) -> Self {
        Self {
            content: Content::with_text(&value),
            rules: None,
            error: None,
        }
    }

    pub fn rules(mut self, rules: impl IntoIterator<Item = TextAreaFieldRule>) -> Self {
        let collected_rules = rules.into_iter().collect::<Vec<_>>().into_boxed_slice();

        self.rules = Some(collected_rules);
        self
    }

    /// Perform the specified `action` in the `TextArea`.
    ///
    /// Returns true, if the performed action updated the textual state; i.e. if validation makes
    /// sense to run or not.
    pub fn perform(&mut self, action: Action) -> bool {
        let should_validate = matches!(action, Action::Edit(_));
        self.content.perform(action);

        should_validate
    }

    pub fn content(&self) -> &Content {
        &self.content
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn is_required(&self) -> bool {
        let Some(ref rules) = self.rules else {
            return false;
        };

        rules
            .iter()
            .any(|rule| rule == &TextAreaFieldRule::Required)
    }

    /// Validate the current content of `Self` against the defined rules.
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
                debug!("setting validation error: {:?}", err);
                self.error = Some(err.to_string());
                return false;
            }
        }

        true
    }

    fn check_rule(&self, rule: TextAreaFieldRule) -> Result<(), ValidationError> {
        // Always pass if a zero length string is not required.
        if self.content.is_empty() && !self.is_required() {
            return Ok(());
        }

        let text = self.content.text();

        match rule {
            TextAreaFieldRule::Required => {
                if text.is_empty() {
                    return Err(ValidationError::Required);
                }
            }
            TextAreaFieldRule::Min(min) => {
                if text.len() < min {
                    return Err(ValidationError::Short(min));
                }
            }
            TextAreaFieldRule::Max(max) => {
                if text.len() > max {
                    return Err(ValidationError::Long(max));
                }
            }
            TextAreaFieldRule::Between(min, max) => {
                if text.len() < min || text.len() > max {
                    return Err(ValidationError::Between(min, max));
                }
            }
        }

        Ok(())
    }
}
