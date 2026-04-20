/// Rules used to define requirements to the validation of a [`MultiTextField`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MultiTextFieldRule {
    /// Set to required.
    ///
    /// This means any length greater than 0.
    Required,

    /// Requires any input to be unique.
    Unique,

    /// Set the minimum length.
    ///
    /// If you are defining both a lower and upper bound, consider [`MultiTextFieldRule::Between`].
    ///
    /// Note that this rule will only apply to a zero length string if [`MultiTextFieldRule::Required`] is also specified.
    /// This allows for rules that enforce optional lengths, only if a string is defined.
    Min(usize),

    /// Set the maximum length.
    ///
    /// If you are defining both a lower and upper bound, consider [`MultiTextFieldRule::Between`].
    ///
    /// Note that this rule will only apply to a zero length string if [`MultiTextFieldRule::Required`] is also specified.
    /// This allows for rules that enforce optional lengths, only if a string is defined.
    Max(usize),

    /// Set a lower and upper bound on length.
    ///
    /// Note that this rule will only apply to a zero length string if [`MultiTextFieldRule::Required`] is also specified.
    /// This allows for rules that enforce optional lengths, only if a string is defined.
    Between(usize, usize),
}
