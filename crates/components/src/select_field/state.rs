use crate::REQUIRED_ERROR_STR;

#[derive(Debug)]
pub struct SelectFieldState<Value> {
    selected: Option<Value>,
    options: Box<[Value]>,
    required: bool,
    error: Option<&'static str>,
}

impl<Value> SelectFieldState<Value>
where
    Value: Clone,
{
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
        self.error = None;
        self.selected = Some(value);
    }

    /// Get the value of the state, if it is valid, otherwise None.
    ///
    /// This method is most useful for "final" extraction on form submit.
    pub fn try_value(&mut self) -> Option<Value> {
        if self.required && self.selected.is_none() {
            tracing::error!("required field has no value selected");
            self.error = Some(REQUIRED_ERROR_STR);

            return None;
        }

        self.error = None;
        self.selected.clone()
    }

    pub(super) fn selected(&self) -> Option<&Value> {
        self.selected.as_ref()
    }

    pub(super) fn error(&self) -> Option<&str> {
        self.error
    }

    pub(super) fn options(&self) -> &[Value] {
        &self.options
    }

    pub(super) fn is_required(&self) -> bool {
        self.required
    }
}
