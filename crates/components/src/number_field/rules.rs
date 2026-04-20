/// Rules used to define requirements to the validation of a [`NumberInput`] widget.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumberFieldRule {
    /// Set to required.
    ///
    /// This means any value.
    Required,

    /// Set the minimum value.
    ///
    /// If you are defining both a lower and upper bound, consider [`NumberInput::Between`].
    Min(i32),

    /// Set the maximum value.
    ///
    /// If you are defining both a lower and upper bound, consider [`NumberInput::Between`].
    Max(i32),

    /// Set a lower and upper bound on value.
    Between(i32, i32),
}
