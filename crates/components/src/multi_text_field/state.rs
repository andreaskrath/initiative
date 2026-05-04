use crate::REQUIRED_ERROR_STR;

#[derive(Debug, Default)]
pub struct MultiTextFieldState {
    value: String,
    selections: Vec<String>,
    required: bool,
    pub(super) normalize: bool,
    error: Option<&'static str>,
}

impl MultiTextFieldState {
    pub fn new(value: String) -> Self {
        Self {
            value,
            selections: Vec::new(),
            required: false,
            normalize: false,
            error: None,
        }
    }

    /// Whether selections should be normalized when added.
    ///
    /// Normalized in this context simply means lowercased.
    pub fn normalize(mut self, normalize: bool) -> Self {
        self.normalize = normalize;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
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

        // Only add non-empty values.
        if !trimmed_value.is_empty() {
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

    /// Get the value of the state, if it is valid, otherwise None.
    ///
    /// This method is most useful for "final" extraction on form submit.
    pub fn try_value(&mut self) -> Option<Box<[String]>> {
        if self.required && self.selections.is_empty() {
            tracing::debug!("required field is empty");
            self.error = Some(REQUIRED_ERROR_STR);

            return None;
        }

        self.error = None;
        Some(self.selections.clone().into_boxed_slice())
    }

    pub(super) fn is_required(&self) -> bool {
        self.required
    }

    pub(super) fn value(&self) -> &str {
        &self.value
    }

    pub(super) fn selections(&self) -> &[String] {
        &self.selections
    }

    pub(super) fn error(&self) -> Option<&str> {
        self.error
    }
}
