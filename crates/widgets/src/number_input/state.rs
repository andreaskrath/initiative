use super::NumberInputRule;
use crate::ValidationError;

use tracing::debug;

#[derive(Debug, Default, Clone)]
pub struct NumberInputState {
    raw: String,
    parsed: Option<i32>,
    rules: Option<Box<[NumberInputRule]>>,
    error: Option<String>,
}

impl NumberInputState {
    pub fn new(value: Option<i32>) -> Self {
        let raw = value.map(|s| s.to_string()).unwrap_or_default();

        Self {
            raw,
            parsed: value,
            rules: None,
            error: None,
        }
    }

    pub fn rules(mut self, rules: impl IntoIterator<Item = NumberInputRule>) -> Self {
        let collected_rules = rules.into_iter().collect::<Vec<_>>().into_boxed_slice();

        self.rules = Some(collected_rules);
        self
    }

    pub fn is_required(&self) -> bool {
        let Some(ref rules) = self.rules else {
            return false;
        };

        rules.iter().any(|rule| rule == &NumberInputRule::Required)
    }

    pub fn value(&self) -> Option<i32> {
        self.parsed
    }

    pub fn raw_value(&self) -> &str {
        &self.raw
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn set(&mut self, value: String) {
        // Allow negative characters.
        if &value == "-" {
            self.raw = value;

            return;
        }

        // Allow input to be cleared again.
        if value.is_empty() {
            self.raw = value;
            self.parsed = None;

            return;
        }

        // Input is parsed, we assign both raw and parsed.
        if let Ok(parsed) = value.parse() {
            self.raw = value;
            self.parsed = Some(parsed);
        }
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

    fn check_rule(&self, rule: NumberInputRule) -> Result<(), ValidationError> {
        // Always pass if not set and not required.
        if self.parsed.is_none() && !self.is_required() {
            return Ok(());
        }

        match rule {
            NumberInputRule::Required => {
                if self.parsed.is_none() {
                    return Err(ValidationError::Required);
                }
            }
            NumberInputRule::Min(min) => {
                if let Some(value) = self.parsed
                    && value < min
                {
                    return Err(ValidationError::LessThan(min));
                }
            }
            NumberInputRule::Max(max) => {
                if let Some(value) = self.parsed
                    && value > max
                {
                    return Err(ValidationError::GreaterThan(max));
                }
            }
            NumberInputRule::Between(min, max) => {
                if let Some(value) = self.parsed
                    && (value < min || value > max)
                {
                    return Err(ValidationError::BetweenValue(min, max));
                }
            }
        }

        Ok(())
    }
}
