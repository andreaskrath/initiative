use iced::Length::Fill;
use strum::VariantArray;
use types::{MagicSchool, SpellCastingTime, SpellDuration, SpellLevel, SpellRange};
use widgets::{Select, TextArea, TextAreaRule, TextInput, TextInputRule, Toggle};

use crate::view::SpellFormMessage;

pub struct SpellFormData {
    pub name: TextInput<SpellFormMessage>,
    pub school: Select<MagicSchool, SpellFormMessage>,
    pub level: Select<SpellLevel, SpellFormMessage>,
    pub casting_time: Select<SpellCastingTime, SpellFormMessage>,
    pub duration: Select<SpellDuration, SpellFormMessage>,
    pub range: Select<SpellRange, SpellFormMessage>,
    pub description: TextArea<SpellFormMessage>,
    pub at_higher_levels: TextArea<SpellFormMessage>,
    pub ritual: Toggle<SpellFormMessage>,
    pub concentration: Toggle<SpellFormMessage>,
    pub verbal: Toggle<SpellFormMessage>,
    pub somatic: Toggle<SpellFormMessage>,
    pub materials: TextInput<SpellFormMessage>,
}

impl Default for SpellFormData {
    fn default() -> Self {
        Self {
            name: TextInput::new("Name", String::new())
                .rules([TextInputRule::Required, TextInputRule::Max(50)])
                .placeholder("Goblin")
                .on_input(SpellFormMessage::NameChanged),
            school: Select::new(
                "School",
                None,
                MagicSchool::VARIANTS.iter().copied(),
                SpellFormMessage::SchoolSelected,
            )
            .placeholder("Select a school")
            .width(Fill)
            .required(true),
            level: Select::new(
                "Level",
                None,
                SpellLevel::VARIANTS.iter().copied(),
                SpellFormMessage::LevelSelected,
            )
            .placeholder("Select a level")
            .width(Fill)
            .required(true),
            casting_time: Select::new(
                "Casting Time",
                None,
                SpellCastingTime::VARIANTS.iter().copied(),
                SpellFormMessage::CastingTimeSelected,
            )
            .placeholder("Select a casting time")
            .width(Fill)
            .required(true),
            duration: Select::new(
                "Duration",
                None,
                SpellDuration::VARIANTS.iter().copied(),
                SpellFormMessage::DurationSelected,
            )
            .placeholder("Select a duration")
            .width(Fill)
            .required(true),
            range: Select::new(
                "Range",
                None,
                SpellRange::VARIANTS.iter().copied(),
                SpellFormMessage::RangeSelected,
            )
            .placeholder("Select a range")
            .width(Fill)
            .required(true),
            description: TextArea::new("Description", "", SpellFormMessage::DescriptionChanged)
                .rules([TextAreaRule::Required])
                .height(300),
            at_higher_levels: TextArea::new(
                "At Higher Levels",
                "",
                SpellFormMessage::AtHigherLevelsChanged,
            )
            .height(100),
            ritual: Toggle::new("Ritual", false).on_toggle(Some(SpellFormMessage::RitualToggled)),
            concentration: Toggle::new("Concentration", false)
                .on_toggle(Some(SpellFormMessage::ConcentrationToggled)),
            verbal: Toggle::new("Verbal", false).on_toggle(Some(SpellFormMessage::VerbalToggled)),
            somatic: Toggle::new("Somatic", false)
                .on_toggle(Some(SpellFormMessage::SomaticToggled)),
            materials: TextInput::new("Materials", String::new())
                .rules([TextInputRule::Max(200)])
                .placeholder("")
                .on_input(SpellFormMessage::MaterialsChanged),
        }
    }
}
