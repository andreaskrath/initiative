mod message;

use components::Field;
use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{container, pick_list, row, text_input},
};
use strum::VariantArray;
use tracing::debug;
use types::{FormMode, MagicSchool, Spell, SpellLevel};

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
            SpellFormMessage::SchoolSelected(magic_school) => self.school = Some(magic_school),
            SpellFormMessage::LevelSelected(spell_level) => self.level = Some(spell_level),
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
        let level = Field::new("School", level_input)
            .required(true)
            .error("Bad bad boy");

        let first_row = row![name, school, level].spacing(10);

        container(first_row).width(1200).into()
    }
}
