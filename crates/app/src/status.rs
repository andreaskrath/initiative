/// An abstraction over a `State` that requires a loading process.
///
/// `Loader` is a loading orchestrator, that at some lator point indirectly resolves to `State`.
///
/// # Notes
///
/// Typically, a `match` should cover a tuple of `(Self, Message)` in `update` methods,
/// as not all `Message` variants will have value for both the `Loader` and `State`.
pub enum Status<Loader, State> {
    /// The `State` is loading, and does not yet have a complete representation.
    Loading(Box<Loader>),

    /// The `State` is ready to be accessed.
    Ready(Box<State>),
}
