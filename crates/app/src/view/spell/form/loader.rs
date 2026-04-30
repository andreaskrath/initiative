use storage::Error;
use storage::repositories::Repository;
use storage::repositories::options::Variant;

use iced::Task;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum LoadMessage {
    OptionsLoaded(Variant, Result<Box<[String]>, Error>),
}

pub struct Loader {
    /// The number of tasks in total the loader must complete.
    pub total: usize,

    /// The number of tasks that have been completed.
    pub progress: usize,

    pub error: Option<Error>,

    pub schools: Option<Box<[String]>>,
    pub levels: Option<Box<[String]>>,
    pub casting_times: Option<Box<[String]>>,
    pub durations: Option<Box<[String]>>,
    pub ranges: Option<Box<[String]>>,
    pub areas: Option<Box<[String]>>,
}

impl Loader {
    pub fn new(repository: Arc<dyn Repository>) -> (Self, Task<LoadMessage>) {
        let variants = &[
            Variant::School,
            Variant::Level,
            Variant::CastingTime,
            Variant::Duration,
            Variant::Range,
            Variant::Area,
        ];

        let mut tasks = Vec::with_capacity(variants.len());
        for variant in variants {
            let task = Task::perform(load_options(repository.clone(), *variant), |result| {
                LoadMessage::OptionsLoaded(*variant, result)
            });

            tasks.push(task);
        }

        let loader = Self {
            total: tasks.len(),
            progress: 0,
            error: None,
            schools: None,
            levels: None,
            casting_times: None,
            durations: None,
            ranges: None,
            areas: None,
        };

        (loader, Task::batch(tasks))
    }

    pub fn update(&mut self, message: LoadMessage) {
        match message {
            LoadMessage::OptionsLoaded(variant, Ok(options)) => match variant {
                Variant::School => self.schools = Some(options),
                Variant::Level => self.levels = Some(options),
                Variant::CastingTime => self.casting_times = Some(options),
                Variant::Duration => self.durations = Some(options),
                Variant::Range => self.ranges = Some(options),
                Variant::Area => self.areas = Some(options),
            },
            LoadMessage::OptionsLoaded(variant, Err(err)) => {
                tracing::error!("failed to load options for '{:?}': {:?}", variant, err);
                self.error = Some(err);
            }
        }

        self.progress += 1;
    }

    pub fn is_done(&self) -> bool {
        self.progress == self.total
    }
}

async fn load_options(
    repository: Arc<dyn Repository>,
    variant: Variant,
) -> Result<Box<[String]>, Error> {
    repository.list_options(variant).await
}
