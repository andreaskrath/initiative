use crate::REQUIRED_ERROR_STR;

#[derive(Debug, Default, Clone)]
pub struct NumberFieldState {
    raw: String,
    parsed: Option<i32>,
    required: bool,
    error: Option<&'static str>,
}

impl NumberFieldState {
    pub fn new(value: Option<i32>) -> Self {
        let raw = value.map(|s| s.to_string()).unwrap_or_default();

        Self {
            raw,
            parsed: value,
            required: false,
            error: None,
        }
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn set(&mut self, value: String) {
        self.error = None;

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

    /// Get the value of the state, if validation succeds, otherwise None.
    ///
    /// This method is most useful for "final" extraction on form submit.
    pub fn try_value(&mut self) -> Option<i32> {
        if self.required && self.parsed.is_none() {
            tracing::error!("required field is empty");
            self.error = Some(REQUIRED_ERROR_STR);

            return None;
        }

        self.error = None;
        self.parsed
    }

    pub(super) fn is_required(&self) -> bool {
        self.required
    }

    pub(super) fn raw_value(&self) -> &str {
        &self.raw
    }

    pub(super) fn error(&self) -> Option<&str> {
        self.error
    }
}
