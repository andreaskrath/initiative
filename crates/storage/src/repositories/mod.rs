pub mod options;

use options::OptionsRepository;

use std::fmt::Debug;

/// A super-trait marker requiring all repositories to be implemented.
pub trait Repository: OptionsRepository + Debug + Send + Sync + 'static {}
