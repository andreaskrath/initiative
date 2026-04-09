/// Rules used to define requirements to the validation of a [`NumberInput`] widget.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumberInputRule {
    /// Set to required.
    ///
    /// This means any value.
    Required,

    /// Set the minimum length.
    ///
    /// If you are defining both a lower and upper bound, consider [`TextInputRule::Between`].
    Min(i32),

    /// Set the maximum length.
    ///
    /// If you are defining both a lower and upper bound, consider [`TextInputRule::Between`].
    Max(i32),

    /// Set a lower and upper bound on value.
    Between(i32, i32),
}
