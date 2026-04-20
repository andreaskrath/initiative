use super::MultiTextFieldRule;
use crate::ValidationError;

use tracing::debug;

#[derive(Debug, Default)]
pub struct MultiTextFieldState {
    value: String,
    selections: Vec<String>,
    rules: Option<Box<[MultiTextFieldRule]>>,
    pub(super) normalize: bool,
    error: Option<String>,
}

impl MultiTextFieldState {
    pub fn new(value: String) -> Self {
        Self {
            value,
            selections: Vec::new(),
            rules: None,
            normalize: false,
            error: None,
        }
    }

    pub fn rules(mut self, rules: impl IntoIterator<Item = MultiTextFieldRule>) -> Self {
        let collected_rules = rules.into_iter().collect::<Vec<_>>().into_boxed_slice();

        self.rules = Some(collected_rules);
        self
    }

    /// Whether selections should be normalized when added.
    ///
    /// Normalized in this context simply means lowercased.
    pub fn normalize(mut self, normalize: bool) -> Self {
        self.normalize = normalize;
        self
    }

    pub fn is_required(&self) -> bool {
        let Some(ref rules) = self.rules else {
            return false;
        };

        rules
            .iter()
            .any(|rule| rule == &MultiTextFieldRule::Required)
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn selections(&self) -> &[String] {
        &self.selections
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// Set the value of the current input.
    pub fn set_value(&mut self, value: String) {
        self.value = value;
        self.error = None;
    }

    /// Add the current value to the collection of selected values.
    pub fn add_selection(&mut self) {
        let start = self.value.len() - self.value.trim_start().len();
        let end = self.value.trim_end().len();
        let trimmed_value = &self.value[start..end];

        // Set error first, to always update error state correctly
        self.error = self.validate(trimmed_value);

        // Then check error state if value should be added or not
        if self.error.is_none() {
            let mut value = std::mem::take(&mut self.value);

            // Remove leading and trailing whitespaces, starting with trailing.
            // Otherwise, the leading indexes are offset incorrectly.
            value.truncate(end);
            value.drain(..start);
            self.selections.push(value);
        }
    }

    pub fn remove_selection(&mut self, index: usize) {
        self.selections.remove(index);
    }

    fn validate(&self, value: &str) -> Option<String> {
        let Some(rules) = &self.rules else {
            return None;
        };

        for rule in rules {
            if let Err(err) = self.check_rule(*rule, value) {
                debug!("setting validation error: {:?}", err);
                return Some(err.to_string());
            }
        }

        None
    }

    fn check_rule(&self, rule: MultiTextFieldRule, value: &str) -> Result<(), ValidationError> {
        match rule {
            MultiTextFieldRule::Required => {
                if value.is_empty() {
                    return Err(ValidationError::Required);
                }
            }
            MultiTextFieldRule::Unique => {
                if self.selections.iter().any(|s| s == value) {
                    return Err(ValidationError::Unique);
                }
            }
            MultiTextFieldRule::Min(min) => {
                if value.len() < min {
                    return Err(ValidationError::Short(min));
                }
            }
            MultiTextFieldRule::Max(max) => {
                if value.len() > max {
                    return Err(ValidationError::Long(max));
                }
            }
            MultiTextFieldRule::Between(min, max) => {
                if value.len() < min || value.len() > max {
                    return Err(ValidationError::Between(min, max));
                }
            }
        }

        Ok(())
    }
}
