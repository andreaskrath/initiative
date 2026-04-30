use types::MagicSchool;
use types::SPELLCASTING_CLASSES;
use types::SpellArea;
use types::SpellCastingTime;
use types::SpellDuration;
use types::SpellLevel;
use types::SpellRange;

use iced::Task;
use std::time::Duration;
use strum::VariantArray;

#[derive(Debug, Clone)]
pub enum LoadMessage {
    Schools(Box<[String]>),
    Levels(Box<[String]>),
    Classes(Box<[String]>),
    CastingTimes(Box<[String]>),
    Durations(Box<[String]>),
    Ranges(Box<[String]>),
    Areas(Box<[String]>),
}

pub struct Loader {
    /// The number of tasks in total the loader must complete.
    pub total: usize,

    /// The number of tasks that have been completed.
    pub progress: usize,

    pub schools: Option<Box<[String]>>,
    pub levels: Option<Box<[String]>>,
    pub classes: Option<Box<[String]>>,
    pub casting_times: Option<Box<[String]>>,
    pub durations: Option<Box<[String]>>,
    pub ranges: Option<Box<[String]>>,
    pub areas: Option<Box<[String]>>,
}

impl Loader {
    pub fn new() -> (Self, Task<LoadMessage>) {
        let tasks = vec![
            Task::perform(load_schools(), LoadMessage::Schools),
            Task::perform(load_levels(), LoadMessage::Levels),
            Task::perform(load_classes(), LoadMessage::Classes),
            Task::perform(load_casting_times(), LoadMessage::CastingTimes),
            Task::perform(load_durations(), LoadMessage::Durations),
            Task::perform(load_ranges(), LoadMessage::Ranges),
            Task::perform(load_areas(), LoadMessage::Areas),
        ];

        let loader = Self {
            total: tasks.len(),
            progress: 0,
            schools: None,
            levels: None,
            classes: None,
            casting_times: None,
            durations: None,
            ranges: None,
            areas: None,
        };

        (loader, Task::batch(tasks))
    }

    pub fn update(&mut self, message: LoadMessage) {
        match message {
            LoadMessage::Schools(schools) => self.schools = Some(schools),
            LoadMessage::Levels(levels) => self.levels = Some(levels),
            LoadMessage::Classes(classes) => self.classes = Some(classes),
            LoadMessage::CastingTimes(casting_times) => self.casting_times = Some(casting_times),
            LoadMessage::Durations(durations) => self.durations = Some(durations),
            LoadMessage::Ranges(ranges) => self.ranges = Some(ranges),
            LoadMessage::Areas(areas) => self.areas = Some(areas),
        }

        self.progress += 1;
    }

    pub fn is_done(&self) -> bool {
        self.progress == self.total
    }
}

async fn load_schools() -> Box<[String]> {
    tokio::time::sleep(Duration::from_secs(1)).await;

    let strings: Vec<_> = MagicSchool::VARIANTS
        .iter()
        .map(|s| s.to_string())
        .collect();

    strings.into_boxed_slice()
}

async fn load_levels() -> Box<[String]> {
    tokio::time::sleep(Duration::from_secs(2)).await;
    let strings: Vec<_> = SpellLevel::VARIANTS.iter().map(|s| s.to_string()).collect();

    strings.into_boxed_slice()
}

async fn load_classes() -> Box<[String]> {
    tokio::time::sleep(Duration::from_secs(3)).await;
    let strings: Vec<_> = SPELLCASTING_CLASSES.iter().map(|s| s.to_string()).collect();

    strings.into_boxed_slice()
}

async fn load_casting_times() -> Box<[String]> {
    tokio::time::sleep(Duration::from_secs(4)).await;
    let strings: Vec<_> = SpellCastingTime::VARIANTS
        .iter()
        .map(|s| s.to_string())
        .collect();

    strings.into_boxed_slice()
}

async fn load_durations() -> Box<[String]> {
    tokio::time::sleep(Duration::from_secs(5)).await;
    let strings: Vec<_> = SpellDuration::VARIANTS
        .iter()
        .map(|s| s.to_string())
        .collect();

    strings.into_boxed_slice()
}

async fn load_ranges() -> Box<[String]> {
    tokio::time::sleep(Duration::from_secs(6)).await;
    let strings: Vec<_> = SpellRange::VARIANTS.iter().map(|s| s.to_string()).collect();

    strings.into_boxed_slice()
}

async fn load_areas() -> Box<[String]> {
    tokio::time::sleep(Duration::from_secs(7)).await;
    let strings: Vec<_> = SpellArea::VARIANTS.iter().map(|s| s.to_string()).collect();

    strings.into_boxed_slice()
}
