pub mod options;
pub mod spells;

use options::OptionsRepository;
use spells::SpellsRepository;

use std::fmt::Debug;

/// A super-trait marker requiring all repositories to be implemented.
pub trait Repository: OptionsRepository + Debug + Send + Sync + 'static {}
