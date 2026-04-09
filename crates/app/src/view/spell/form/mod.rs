mod fields;
mod message;

pub use message::SpellFormMessage;

use crate::{
    message::Message,
    view::{
        ViewContent,
        spell::form::fields::{SpellFormFields, SpellMaterialInput},
    },
};
use style::button::ButtonClass;
use widgets::Element;

use iced::{
    Alignment,
    Length::Fill,
    Task,
    widget::{self, Column, column, row},
};
use tracing::debug;
use types::FormMode;

pub struct SpellForm {
    mode: FormMode,
    fields: SpellFormFields,
}

impl<'a> SpellForm {
    pub fn new(mode: FormMode) -> Self {
        Self {
            mode,
            fields: SpellFormFields::default(),
        }
    }

    fn header(&'a self) -> Element<'a, SpellFormMessage> {
        let title = widgets::text::view_title("Spell Forge");

        let sub_title = widgets::text::view_sub_title("Inscribe thy arcane workings");

        column![title, sub_title]
            .width(Fill)
            .align_x(Alignment::Center)
            .into()
    }

    fn identification(&'a self) -> Element<'a, SpellFormMessage> {
        let title = widgets::text::heading("IDENTIFICATION");

        let name = widgets::text_input(Some("Name"), &self.fields.name)
            .placeholder("Goblin")
            .on_input(SpellFormMessage::NameChanged);

        let school = widgets::select(
            "School",
            &self.fields.school,
            SpellFormMessage::SchoolSelected,
        )
        .placeholder("Select a magic school")
        .width(Fill);

        let level = widgets::select("Level", &self.fields.level, SpellFormMessage::LevelSelected)
            .placeholder("Select a spell level")
            .width(Fill);

        let row = row![name, school, level].spacing(10);

        column![title, row]
            .align_x(Alignment::Center)
            .spacing(10)
            .into()
    }

    fn casting_properties(&'a self) -> Element<'a, SpellFormMessage> {
        let title = widgets::text::heading("CASTING PROPERTIES");

        let casting_time = widgets::select(
            "Casting Time",
            &self.fields.casting_time,
            SpellFormMessage::CastingTimeSelected,
        )
        .placeholder("Select a casting time")
        .width(Fill);

        let duration = widgets::select(
            "Duration",
            &self.fields.duration,
            SpellFormMessage::DurationSelected,
        )
        .placeholder("Select a duration")
        .width(Fill);

        let range = widgets::select("Range", &self.fields.range, SpellFormMessage::RangeSelected)
            .placeholder("Select a range")
            .width(Fill);

        let verbal = widgets::toggle("Verbal", self.fields.verbal)
            .width(Fill)
            .on_toggle(SpellFormMessage::VerbalToggled);

        let somatic = widgets::toggle("Somatic", self.fields.somatic)
            .width(Fill)
            .on_toggle(SpellFormMessage::SomaticToggled);

        let ritual = widgets::toggle("Ritual", self.fields.ritual)
            .width(Fill)
            .on_toggle(SpellFormMessage::RitualToggled);

        let concentration = widgets::toggle("Concentration", self.fields.concentration)
            .width(Fill)
            .on_toggle(SpellFormMessage::ConcentrationToggled);

        let add_materials_button = widget::container(
            widget::button("Add Material")
                .class(ButtonClass::Primary)
                .on_press(SpellFormMessage::AddMaterial),
        )
        .align_right(Fill);

        let mut materials = Column::with_capacity(self.fields.materials.len()).spacing(10);
        for (index, spell_material) in self.fields.materials.iter().enumerate() {
            let material = widgets::text_input(Some(&spell_material.label), &spell_material.material)
                .placeholder("A bat wing")
                .on_input(move |new_material| {
                    SpellFormMessage::MaterialChanged(index, new_material)
                });
            let consumed = widgets::toggle("Consumed", spell_material.consumed)
                .on_toggle(SpellFormMessage::MaterialConsumed(index));
            let remove = widget::button(widgets::text::display("Remove"))
                .class(ButtonClass::Danger)
                .on_press(SpellFormMessage::RemoveMaterial(index));

            let spell_material_input = row![material, consumed, remove]
                .spacing(10)
                .align_y(Alignment::End);

            materials = materials.push(spell_material_input);
        }

        let first_row = row![casting_time, duration, range].spacing(10);

        let second_row = row![verbal, somatic, concentration, ritual,].spacing(10);

        column![
            title,
            first_row,
            second_row,
            add_materials_button,
            materials,
        ]
        .align_x(Alignment::Center)
        .spacing(10)
        .into()
    }

    fn effect(&'a self) -> Element<'a, SpellFormMessage> {
        let title = widgets::text::heading("EFFECT");

        let description = widgets::text_area(
            "Description",
            &self.fields.description,
            SpellFormMessage::DescriptionChanged,
        )
        .height(300);

        let at_higher_levels = widgets::text_area(
            "At Higher Levels",
            &self.fields.at_higher_levels,
            SpellFormMessage::AtHigherLevelsChanged,
        )
        .height(100);

        column![title, description, at_higher_levels]
            .align_x(Alignment::Center)
            .spacing(10)
            .into()
    }

    fn quote(&'a self) -> Element<'a, SpellFormMessage> {
        let title = widgets::text::heading("QUOTE");

        let text = widgets::text_area(
            "Text",
            &self.fields.quote_text,
            SpellFormMessage::QuoteTextChanged,
        )
        .placeholder("Something")
        .height(100);

        let source = widgets::text_input(Some("Source"), &self.fields.quote_source)
            .placeholder("Someone")
            .on_input(SpellFormMessage::QuoteSourceChanged);

        column![title, text, source,]
            .align_x(Alignment::Center)
            .spacing(10)
            .into()
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
            SpellFormMessage::SchoolSelected(school) => {
                self.fields.school.set(school);
            }
            SpellFormMessage::LevelSelected(level) => {
                self.fields.level.set(level);
            }
            SpellFormMessage::CastingTimeSelected(casting_time) => {
                self.fields.casting_time.set(casting_time);
            }
            SpellFormMessage::DurationSelected(duration) => {
                self.fields.duration.set(duration);
            }
            SpellFormMessage::RangeSelected(range) => {
                self.fields.range.set(range);
            }
            SpellFormMessage::DescriptionChanged(action) => {
                if self.fields.description.perform(action) {
                    self.fields.description.validate();
                }
            }
            SpellFormMessage::AtHigherLevelsChanged(action) => {
                self.fields.at_higher_levels.perform(action);
            }
            SpellFormMessage::RitualToggled => self.fields.ritual = !self.fields.ritual,
            SpellFormMessage::ConcentrationToggled => {
                self.fields.concentration = !self.fields.concentration;
            }
            SpellFormMessage::VerbalToggled => self.fields.verbal = !self.fields.verbal,
            SpellFormMessage::SomaticToggled => self.fields.somatic = !self.fields.somatic,
            SpellFormMessage::AddMaterial => {
                self.fields
                    .materials
                    .push(SpellMaterialInput::new(self.fields.materials.len() + 1));
            }
            SpellFormMessage::RemoveMaterial(index) => {
                self.fields.materials.remove(index);

                for (i, mat) in self.fields.materials.iter_mut().enumerate() {
                    mat.label = format!("Material {}", i + 1);
                }
            }
            SpellFormMessage::MaterialChanged(index, material) => {
                if let Some(spell_material) = self.fields.materials.get_mut(index) {
                    spell_material.material.set(material);
                    spell_material.material.validate();
                }
            }
            SpellFormMessage::MaterialConsumed(index) => {
                if let Some(spell_material) = self.fields.materials.get_mut(index) {
                    spell_material.consumed = !spell_material.consumed;
                }
            }
            SpellFormMessage::QuoteTextChanged(action) => {
                self.fields.quote_text.perform(action);
                self.fields.quote_text.validate();
            }
            SpellFormMessage::QuoteSourceChanged(quote_source) => {
                self.fields.quote_source.set(quote_source);
                self.fields.quote_source.validate();
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Self::ContentMessage> {
        let header = self.header();
        let identification = self.identification();
        let casting_properties = self.casting_properties();
        let effect = self.effect();
        let quote = self.quote();

        let view = column![
            header,
            identification,
            casting_properties,
            effect,
            quote,
        ]
        .align_x(Alignment::Center)
        .spacing(50);

        widget::scrollable(view).into()
    }
}
