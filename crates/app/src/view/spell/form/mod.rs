mod data;
mod message;

use iced::{
    Alignment, Element, Task,
    widget::{column, container, row},
};
use tracing::debug;
use types::FormMode;

use crate::{
    message::Message,
    tab::{TabContent, TabId},
    view::spell::form::data::SpellFormData,
};

pub use message::SpellFormMessage;

pub struct SpellForm {
    id: TabId,
    mode: FormMode,
    data: SpellFormData,
}

impl SpellForm {
    pub fn new(id: TabId, mode: FormMode) -> Self {
        Self {
            id,
            mode,
            data: SpellFormData::default(),
        }
    }
}

impl TabContent for SpellForm {
    type ContentMessage = SpellFormMessage;

    fn id(&self) -> crate::tab::TabId {
        self.id
    }

    fn title(&self) -> &str {
        match self.mode {
            FormMode::Create => "Create spell",
            FormMode::Edit(_) => "Edit spell",
        }
    }

    fn update(&mut self, message: Self::ContentMessage) -> Task<Message> {
        debug!("updating spell form with: {:?}", message);

        match message {
            SpellFormMessage::NameChanged(name) => {
                self.data.name.set(name);
                self.data.name.validate();
            }
            SpellFormMessage::SchoolSelected(school) => self.data.school.select(Some(school)),
            SpellFormMessage::LevelSelected(level) => self.data.level.select(Some(level)),
            SpellFormMessage::CastingTimeSelected(casting_time) => {
                self.data.casting_time.select(Some(casting_time));
            }
            SpellFormMessage::DurationSelected(duration) => {
                self.data.duration.select(Some(duration))
            }
            SpellFormMessage::RangeSelected(range) => self.data.range.select(Some(range)),
            SpellFormMessage::DescriptionChanged(action) => self.data.description.update(action),
            SpellFormMessage::AtHigherLevelsChanged(action) => {
                self.data.at_higher_levels.update(action);
            }
            SpellFormMessage::RitualToggled => self.data.ritual.toggle(),
            SpellFormMessage::ConcentrationToggled => self.data.concentration.toggle(),
            SpellFormMessage::VerbalToggled => self.data.verbal.toggle(),
            SpellFormMessage::SomaticToggled => self.data.somatic.toggle(),
            SpellFormMessage::MaterialsChanged(materials) => self.data.materials.set(materials),
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Self::ContentMessage> {
        let name = self.data.name.view();

        let school = self.data.school.view();

        let level = self.data.level.view();

        let casting_time = self.data.casting_time.view();

        let duration = self.data.duration.view();

        let range = self.data.range.view();

        let first_row = row![name, school, level].spacing(10);
        let second_row = row![casting_time, duration, range].spacing(10);

        let description = self.data.description.view();

        let at_higher_levels = self.data.at_higher_levels.view();

        let ritual = self.data.ritual.view();

        let concentration = self.data.concentration.view();

        let verbal = self.data.verbal.view();

        let somatic = self.data.somatic.view();

        let materials = self.data.materials.view();

        let third_row = row![verbal, somatic, materials, ritual, concentration]
            .spacing(10)
            .align_y(Alignment::End);

        let form = column![
            first_row,
            second_row,
            description,
            at_higher_levels,
            third_row
        ]
        .spacing(10);

        container(form).width(1200).into()
    }
}
