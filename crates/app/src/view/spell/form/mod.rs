mod message;

use components::Field;
use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{
        checkbox, column, container, pick_list, row, text_editor, text_editor::Content, text_input,
    },
};
use strum::VariantArray;
use tracing::debug;
use types::{
    FormMode, MagicSchool, Spell, SpellCastingTime, SpellDuration, SpellLevel, SpellRange,
};

use crate::{
    message::Message,
    tab::{TabContent, TabId},
};

pub use message::SpellFormMessage;

pub struct SpellForm {
    id: TabId,
    mode: FormMode,
    spell: Spell,
    name: String,
    school: Option<MagicSchool>,
    level: Option<SpellLevel>,
    casting_time: Option<SpellCastingTime>,
    duration: Option<SpellDuration>,
    range: Option<SpellRange>,
    description_content: Content,
    at_higher_levels_content: Content,
    ritual: bool,
    concentration: bool,
    verbal: bool,
    somatic: bool,
    materials: String,
}

impl SpellForm {
    pub fn new(id: TabId, mode: FormMode) -> Self {
        Self {
            id,
            mode,
            spell: Spell::default(),
            name: String::new(),
            school: None,
            level: None,
            casting_time: None,
            duration: None,
            range: None,
            description_content: Content::new(),
            at_higher_levels_content: Content::new(),
            ritual: false,
            concentration: false,
            verbal: false,
            somatic: false,
            materials: String::new(),
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
            SpellFormMessage::NameChanged(name) => self.name = name,
            SpellFormMessage::SchoolSelected(school) => self.school = Some(school),
            SpellFormMessage::LevelSelected(level) => self.level = Some(level),
            SpellFormMessage::CastingTimeSelected(casting_time) => {
                self.casting_time = Some(casting_time)
            }
            SpellFormMessage::DurationSelected(duration) => self.duration = Some(duration),
            SpellFormMessage::RangeSelected(range) => self.range = Some(range),
            SpellFormMessage::DescriptionChanged(action) => {
                self.description_content.perform(action)
            }
            SpellFormMessage::AtHigherLevelsChanged(action) => {
                self.at_higher_levels_content.perform(action)
            }
            SpellFormMessage::RitualToggled(ritual) => self.ritual = ritual,
            SpellFormMessage::ConcentrationToggled(concentration) => {
                self.concentration = concentration
            }
            SpellFormMessage::VerbalToggled(verbal) => self.verbal = verbal,
            SpellFormMessage::SomaticToggled(somatic) => self.somatic = somatic,
            SpellFormMessage::MaterialsChanged(materials) => self.materials = materials,
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Self::ContentMessage> {
        let name_input = text_input("Goblin", &self.name).on_input(SpellFormMessage::NameChanged);
        let name = Field::new("Name", name_input)
            .required(true)
            .error("Bad bad boy");

        let school_input = pick_list(
            MagicSchool::VARIANTS,
            self.school,
            SpellFormMessage::SchoolSelected,
        )
        .placeholder("Select a school")
        .width(Fill);
        let school = Field::new("School", school_input)
            .required(true)
            .error("Bad bad boy");

        let level_input = pick_list(
            SpellLevel::VARIANTS,
            self.level,
            SpellFormMessage::LevelSelected,
        )
        .placeholder("Select a level")
        .width(Fill);
        let level = Field::new("Level", level_input)
            .required(true)
            .error("Bad bad boy");

        let casting_time_input = pick_list(
            SpellCastingTime::VARIANTS,
            self.casting_time,
            SpellFormMessage::CastingTimeSelected,
        )
        .placeholder("Select a casting time")
        .width(Fill);
        let casting_time = Field::new("Casting time", casting_time_input)
            .required(true)
            .error("Bad bad boy");

        let duration_input = pick_list(
            SpellDuration::VARIANTS,
            self.duration,
            SpellFormMessage::DurationSelected,
        )
        .placeholder("Select a duration")
        .width(Fill);
        let duration = Field::new("Duration", duration_input)
            .required(true)
            .error("Bad bad boy");

        let range_input = pick_list(
            SpellRange::VARIANTS,
            self.range,
            SpellFormMessage::RangeSelected,
        )
        .placeholder("Select a range")
        .width(Fill);
        let range = Field::new("Range", range_input)
            .required(true)
            .error("bad bad boy");

        let first_row = row![name, school, level].spacing(10);
        let second_row = row![casting_time, duration, range].spacing(10);

        let description_input = text_editor(&self.description_content)
            .placeholder("Placeholder")
            .on_action(SpellFormMessage::DescriptionChanged)
            .height(300);
        let description = Field::new("Description", description_input)
            .required(true)
            .error("bad bad boy");

        let at_higher_levels_input = text_editor(&self.at_higher_levels_content)
            .placeholder("Placeholder")
            .on_action(SpellFormMessage::AtHigherLevelsChanged)
            .height(100);
        let at_higher_levels = Field::new("At Higher Levels", at_higher_levels_input);

        let ritual_input = checkbox(self.ritual)
            .label("Ritual")
            .on_toggle(SpellFormMessage::RitualToggled);
        let ritual = Field::new("", ritual_input);

        let concentration_input = checkbox(self.concentration)
            .label("Concentration")
            .on_toggle(SpellFormMessage::ConcentrationToggled);
        let concentration = Field::new("", concentration_input);

        let verbal_input = checkbox(self.verbal)
            .label("Verbal")
            .on_toggle(SpellFormMessage::VerbalToggled);
        let verbal = Field::new("", verbal_input);

        let somatic_input = checkbox(self.somatic)
            .label("Somatic")
            .on_toggle(SpellFormMessage::SomaticToggled);
        let somatic = Field::new("", somatic_input);

        let materials_input =
            text_input("Goblin", &self.materials).on_input(SpellFormMessage::MaterialsChanged);
        let materials = Field::new("Materials", materials_input);

        let third_row = row![verbal, somatic, materials, ritual, concentration].spacing(10);

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
