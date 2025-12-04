use super::error::ValueError;

/// Rules used to define requirements to an instance of [`ValueString`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueStringRule {
    /// Set to required.
    ///
    /// This means any length greater than 0.
    Required,

    /// Set the minimum length.
    ///
    /// If you are defining both a lower and upper bound, consider [`ValueStringRule::Between`].
    ///
    /// Note that this rule will only apply to a zero length string if [`ValueStringRule::Required`] is also specified.
    /// This allows for rules that enforce optional lengths, only if a string is defined.
    Min(usize),

    /// Set the maximum length.
    ///
    /// If you are defining both a lower and upper bound, consider [`ValueStringRule::Between`].
    ///
    /// Note that this rule will only apply to a zero length string if [`ValueStringRule::Required`] is also specified.
    /// This allows for rules that enforce optional lengths, only if a string is defined.
    Max(usize),

    /// Set a lower and upper bound on length.
    ///
    /// Note that this rule will only apply to a zero length string if [`ValueStringRule::Required`] is also specified.
    /// This allows for rules that enforce optional lengths, only if a string is defined.
    Between(usize, usize),
}

/// A `ValueString` is any user input in the form of a string.
///
/// This struct owns the value of the string, and can perform validation of said string, based on
/// the defined `rules` upon creation.
pub struct ValueString {
    /// The value of `Self`.
    value: String,

    /// The first error encountered during validation, if any exists.
    ///
    /// The reason this is stored as a String, and not an actual error type, is to avoid String
    /// allocations during view calls.
    error: Option<String>,

    /// The rules that govern the validation of `value`.
    rules: Box<[ValueStringRule]>,
}

impl ValueString {
    pub fn new(value: String, rules: impl IntoIterator<Item = ValueStringRule>) -> Self {
        let collected_rules = rules
            .into_iter()
            .collect::<Vec<ValueStringRule>>()
            .into_boxed_slice();

        Self {
            value,
            error: None,
            rules: collected_rules,
        }
    }

    /// Set the value of `Self`.
    pub fn set(&mut self, value: String) {
        self.value = value;
    }

    /// Get the value of `Self`.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get the first error encountered during validation, if any exists.
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// Validate the current value of `Self` against the defined rules.
    ///
    /// This methods short-circuits on the first error, and checks rules in the order they were
    /// defined during the creation of this struct.
    pub fn validate(&mut self) -> bool {
        for rule in &self.rules {
            if let Err(err) = self.check_rule(*rule) {
                self.error = Some(err.to_string());
                return false;
            }
        }

        true
    }

    fn is_required(&self) -> bool {
        self.rules
            .iter()
            .any(|rule| rule == &ValueStringRule::Required)
    }

    fn check_rule(&self, rule: ValueStringRule) -> Result<(), ValueError> {
        // Always pass if a zero length string is not required.
        if self.value.is_empty() && !self.is_required() {
            return Ok(());
        }

        match rule {
            ValueStringRule::Required => {
                if self.value.is_empty() {
                    return Err(ValueError::Required);
                }
            }
            ValueStringRule::Min(min) => {
                if self.value.len() < min {
                    return Err(ValueError::Short(min));
                }
            }
            ValueStringRule::Max(max) => {
                if self.value.len() > max {
                    return Err(ValueError::Long(max));
                }
            }
            ValueStringRule::Between(min, max) => {
                if self.value.len() < min || self.value.len() > max {
                    return Err(ValueError::Between(min, max));
                }
            }
        }

        Ok(())
    }
}
