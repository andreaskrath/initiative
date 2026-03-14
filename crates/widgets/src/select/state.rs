use crate::ValidationError;

pub struct SelectState<Value> {
    selected: Option<Value>,
    options: Box<[Value]>,
    required: bool,
    error: Option<String>,
}

impl<Value> SelectState<Value> {
    pub fn new(options: impl IntoIterator<Item = Value>, value: Option<Value>) -> Self {
        let collected_options = options.into_iter().collect::<Vec<_>>().into_boxed_slice();

        Self {
            selected: value,
            options: collected_options,
            required: false,
            error: None,
        }
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn set(&mut self, value: Value) {
        self.selected = Some(value);
    }

    pub fn clear(&mut self) {
        self.selected = None;
    }

    pub fn selected(&self) -> Option<&Value> {
        self.selected.as_ref()
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn options(&self) -> &[Value] {
        &self.options
    }

    pub fn is_required(&self) -> bool {
        self.required
    }

    pub fn validate(&mut self) -> bool {
        if !self.required || self.selected.is_some() {
            true
        } else {
            self.error = Some(ValidationError::Required.to_string());
            false
        }
    }
}
