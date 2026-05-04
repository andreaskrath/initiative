use crate::REQUIRED_ERROR_STR;

#[derive(Debug, Default)]
pub struct TextFieldState {
    value: String,
    required: bool,
    error: Option<&'static str>,
}

impl TextFieldState {
    pub fn new(value: String) -> Self {
        Self {
            value,
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
        self.value = value;
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get the value of the state, if it is valid, otherwise None.
    ///
    /// This method is most useful for "final" extraction on form submit.
    pub fn try_value(&mut self) -> Option<String> {
        let start = self.value.len() - self.value.trim_start().len();
        let end = self.value.trim_end().len();
        let trimmed = &self.value[start..end];

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

    pub(super) fn is_required(&self) -> bool {
        self.required
    }

    pub(super) fn error(&self) -> Option<&str> {
        self.error
    }
}
