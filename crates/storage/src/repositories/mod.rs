pub mod options;

use options::OptionsRepository;

/// A super-trait marker requiring all repositories to be implemented.
pub trait Repository: OptionsRepository + Send + Sync + 'static {}
