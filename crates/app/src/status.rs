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

impl<Loader, State> Status<Loader, State> {
    pub fn as_loading_mut(&mut self) -> Option<&mut Loader> {
        if let Status::Loading(loader) = self {
            Some(loader)
        } else {
            None
        }
    }

    pub fn as_ready_mut(&mut self) -> Option<&mut State> {
        if let Status::Ready(state) = self {
            Some(state)
        } else {
            None
        }
    }
}

/// Unwraps a [`Status`] as [`Status::Ready`], returning a mutable reference to the inner state.
///
/// If the status is not [`Status::Ready`], logs an error and early-returns
/// `(Task::none(), None)` from the enclosing function.
///
/// # Usage
///
/// Only valid inside functions that return `(Task<_>, Option<_>)`.
macro_rules! ready {
    ($self:expr) => {
        match $self.as_ready_mut() {
            Some(state) => state,
            None => {
                tracing::error!("received in non-ready state");

                return (Task::none(), None);
            }
        }
    };
}

pub(crate) use ready;

/// Unwraps a [`Status`] as [`Status::Loading`], returning a mutable reference to the inner loader.
///
/// If the status is not [`Status::Loading`], logs an error and early-returns
/// `(Task::none(), None)` from the enclosing function.
///
/// # Usage
///
/// Only valid inside functions that return `(Task<_>, Option<_>)`.
macro_rules! loading {
    ($self:expr) => {
        match $self.as_loading_mut() {
            Some(loader) => loader,
            None => {
                tracing::error!("received in non-loader state");

                return (Task::none(), None);
            }
        }
    };
}

pub(crate) use loading;
