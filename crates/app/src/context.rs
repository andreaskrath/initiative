use storage::repositories::Repository;
use storage::repositories::options::Options;
use storage::repositories::options::OptionsRepository;
use style::theme::Theme;
use style::theme::variant::ThemeVariant;

use std::sync::Arc;

#[derive(Debug)]
struct InnerState {
    theme: Theme,
    repository: Box<dyn Repository>,
}

#[derive(Debug, Clone)]
pub struct Context {
    inner: Arc<InnerState>,
}

impl Context {
    pub fn new(repository: impl Repository) -> Self {
        let inner = InnerState {
            theme: ThemeVariant::default().into(),
            repository: Box::new(repository),
        };

        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn theme(&self) -> Theme {
        self.inner.theme.clone()
    }
}

impl OptionsRepository for Context {
    fn options(&self) -> &dyn Options {
        self.inner.repository.options()
    }
}
