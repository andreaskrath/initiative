use tracing::debug;

use crate::ValidationError;

use super::TextInputRule;

#[derive(Debug, Default)]
pub struct TextInputState {
    value: String,
    rules: Option<Box<[TextInputRule]>>,
    error: Option<String>,
}

impl TextInputState {
    pub fn new(value: String) -> Self {
        Self {
            value,
            rules: None,
            error: None,
        }
    }

    pub fn rules(mut self, rules: impl IntoIterator<Item = TextInputRule>) -> Self {
        let collected_rules = rules.into_iter().collect::<Vec<_>>().into_boxed_slice();

        self.rules = Some(collected_rules);
        self
    }

    pub fn is_required(&self) -> bool {
        let Some(ref rules) = self.rules else {
            return false;
        };

        rules.iter().any(|rule| rule == &TextInputRule::Required)
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn set(&mut self, value: String) {
        self.value = value;
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
                debug!("setting validation error: {:?}", err);
                self.error = Some(err.to_string());
                return false;
            }
        }

        true
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
