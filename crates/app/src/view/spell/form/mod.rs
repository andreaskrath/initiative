mod fields;
mod message;

use iced::{
    Alignment, Element, Task,
    widget::{column, row},
};
use tracing::debug;
use types::FormMode;

use crate::{
    message::Message,
    view::{ViewContent, spell::form::fields::SpellFormFields},
};

pub use message::SpellFormMessage;

pub struct SpellForm {
    mode: FormMode,
    fields: SpellFormFields,
}

impl SpellForm {
    pub fn new(mode: FormMode) -> Self {
        Self {
            mode,
            fields: SpellFormFields::default(),
        }
    }
}

impl ViewContent for SpellForm {
    type ContentMessage = SpellFormMessage;

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
                self.fields.name.set(name);
                self.fields.name.validate();
            }
            SpellFormMessage::SchoolSelected(school) => self.fields.school.select(Some(school)),
            SpellFormMessage::LevelSelected(level) => self.fields.level.select(Some(level)),
            SpellFormMessage::CastingTimeSelected(casting_time) => {
                self.fields.casting_time.select(Some(casting_time));
            }
            SpellFormMessage::DurationSelected(duration) => {
                self.fields.duration.select(Some(duration))
            }
            SpellFormMessage::RangeSelected(range) => self.fields.range.select(Some(range)),
            SpellFormMessage::DescriptionChanged(action) => self.fields.description.update(action),
            SpellFormMessage::AtHigherLevelsChanged(action) => {
                self.fields.at_higher_levels.update(action);
            }
            SpellFormMessage::RitualToggled => self.fields.ritual.toggle(),
            SpellFormMessage::ConcentrationToggled => self.fields.concentration.toggle(),
            SpellFormMessage::VerbalToggled => self.fields.verbal.toggle(),
            SpellFormMessage::SomaticToggled => self.fields.somatic.toggle(),
            SpellFormMessage::MaterialsChanged(materials) => self.fields.materials.set(materials),
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Self::ContentMessage> {
        let title = widgets::view_title("Create New Spell");

        let name = self.fields.name.view();

        let school = self.fields.school.view();

        let level = self.fields.level.view();

        let casting_time = self.fields.casting_time.view();

        let duration = self.fields.duration.view();

        let range = self.fields.range.view();

        let first_row = row![name, school, level].spacing(10);
        let second_row = row![casting_time, duration, range].spacing(10);

        let description = self.fields.description.view();

        let at_higher_levels = self.fields.at_higher_levels.view();

        let ritual = self.fields.ritual.view();

        let concentration = self.fields.concentration.view();

        let verbal = self.fields.verbal.view();

        let somatic = self.fields.somatic.view();

        let materials = self.fields.materials.view();

        let third_row = row![verbal, somatic, materials, ritual, concentration]
            .spacing(10)
            .align_y(Alignment::End);

        column![
            title,
            first_row,
            second_row,
            description,
            at_higher_levels,
            third_row
        ]
        .spacing(10)
        .into()
    }
}
