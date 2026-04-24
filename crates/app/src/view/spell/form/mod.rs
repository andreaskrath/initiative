mod fields;
pub mod message;

use crate::view::content::ViewContent;
use crate::view::spell::form::fields::SpellFormFields;
use crate::view::spell::form::fields::SpellMaterialInput;
use crate::view::spell::form::fields::SpellShapeInput;
use crate::view::spell::form::message::SpellFormEffect;
use crate::view::spell::form::message::SpellFormMessage;
use components::image_field::error::ImageError;
use components::label::Label;
use style::layout::BODY_SPACING;
use style::layout::LABEL_SPACING;
use style::layout::SECTION_SPACING;
use types::FormMode;
use types::SPELLCASTING_CLASSES;
use widgets::Element;

use iced::Alignment;
use iced::Length;
use iced::Length::Fill;
use iced::Task;
use iced::widget;
use iced::widget::Column;
use iced::widget::Row;
use iced::widget::column;
use iced::widget::row;

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

    fn title(&'a self) -> Element<'a, SpellFormMessage> {
        let title = components::text::view_title("Spell Forge");

        let sub_title = components::text::view_sub_title("Inscribe thy arcane workings");

        column![title, sub_title]
            .width(Fill)
            .align_x(Alignment::Center)
            .into()
    }

    fn identity_section(&'a self) -> Element<'a, SpellFormMessage> {
        let header = components::form::section_header(
            "IDENTITY",
            "Fundamental classification and class availability.",
        );

        let name = components::text_field(Some("NAME"), &self.fields.name)
            .placeholder("Goblin")
            .on_input(SpellFormMessage::NameChanged);
        let school = components::select_field(
            "SCHOOL",
            &self.fields.school,
            SpellFormMessage::SchoolSelected,
        )
        .placeholder("Select a magic school");
        let level =
            components::select_field("LEVEL", &self.fields.level, SpellFormMessage::LevelSelected)
                .placeholder("Select a spell level");

        let classes: Element<_> = {
            let elements_selected_text = format!("{} classes selected", self.fields.classes.len());
            let elements_selected = components::text::detail(elements_selected_text);
            let label = row![
                Label::new("CLASSES").required(false),
                widget::space::horizontal().width(Length::Fill),
                elements_selected
            ];

            let mut cols = Column::new().spacing(BODY_SPACING);

            for row_variants in SPELLCASTING_CLASSES.chunks(3) {
                let row = Row::with_children(row_variants.iter().map(|class| {
                    components::toggle(class.as_ref(), self.fields.classes.contains(class))
                        .width(Length::Fill)
                        .on_toggle(SpellFormMessage::ClassToggled(*class))
                        .into()
                }))
                .spacing(BODY_SPACING);

                cols = cols.push(row);
            }

            let grid: Element<_> = cols.into();

            column![label, grid].spacing(LABEL_SPACING).into()
        };

        let tags = components::multi_text_field(Some("TAGS"), &self.fields.tags)
            .placeholder("Write a tag")
            .on_input(SpellFormMessage::TagChanged)
            .on_submit(SpellFormMessage::TagSubmitted)
            .on_remove(SpellFormMessage::TagRemoved);

        let classification = row![school, level].spacing(BODY_SPACING);
        let form = column![name, classification, classes, tags].spacing(BODY_SPACING);
        let body = components::form::section_body(form);

        row![header, body].into()
    }

    fn casting_section(&'a self) -> Element<'a, SpellFormMessage> {
        let header = components::form::section_header(
            "CASTING",
            "The mechanics and requirements of casting the spell.",
        );

        let casting_time = components::select_field(
            "CASTING TIME",
            &self.fields.casting_time,
            SpellFormMessage::CastingTimeSelected,
        )
        .placeholder("Select a casting time");

        let properties: Element<_> = {
            let label = Label::new("PROPERTIES");
            let ritual = components::toggle("Ritual", self.fields.ritual)
                .width(Fill)
                .on_toggle(SpellFormMessage::RitualToggled);
            let concentration = components::toggle("Concentration", self.fields.concentration)
                .width(Fill)
                .on_toggle(SpellFormMessage::ConcentrationToggled);
            let toggle_row = row![ritual, concentration].spacing(BODY_SPACING);

            column![label, toggle_row].spacing(LABEL_SPACING).into()
        };

        let components: Element<_> = {
            let label = Label::new("COMPONENTS");
            let verbal = components::toggle("Verbal", self.fields.verbal)
                .width(Fill)
                .on_toggle(SpellFormMessage::VerbalToggled);
            let somatic = components::toggle("Somatic", self.fields.somatic)
                .width(Fill)
                .on_toggle(SpellFormMessage::SomaticToggled);
            let material = components::toggle("Material", self.fields.material)
                .width(Fill)
                .on_toggle(SpellFormMessage::MaterialToggled);
            let components_row = row![verbal, somatic, material].spacing(BODY_SPACING);

            column![label, components_row].spacing(LABEL_SPACING).into()
        };

        let materials: Element<_> = {
            let mut columns =
                Column::with_capacity(self.fields.materials.len()).spacing(BODY_SPACING);

            if self.fields.material {
                for (index, spell_material) in self.fields.materials.iter().enumerate() {
                    let material_label = if index == 0 { Some("MATERIAL") } else { None };
                    let worth_label = if index == 0 { Some("WORTH") } else { None };
                    let material = components::text_field(material_label, &spell_material.material)
                        .placeholder("A bat wing")
                        .on_input(move |new_material| {
                            SpellFormMessage::MaterialChanged(index, new_material)
                        });
                    let worth = components::text_field(worth_label, &spell_material.worth)
                        .placeholder("100 gp")
                        .on_input(move |new_worth| {
                            SpellFormMessage::MaterialWorthChanged(index, new_worth)
                        });
                    let consumed = components::toggle("Consumed", spell_material.consumed)
                        .on_toggle(SpellFormMessage::MaterialConsumed(index));
                    let spell_material_input = row![material, worth, consumed]
                        .spacing(BODY_SPACING)
                        .align_y(Alignment::End);

                    columns = columns.push(spell_material_input);
                }
            }

            columns.into()
        };

        let casting = row![casting_time, properties]
            .spacing(BODY_SPACING)
            .align_y(Alignment::End);
        let form = column![casting, components, materials]
            .align_x(Alignment::Center)
            .spacing(BODY_SPACING);
        let body = components::form::section_body(form);

        row![header, body].into()
    }

    fn effect_section(&'a self) -> Element<'a, SpellFormMessage> {
        let header = components::form::section_header(
            "EFFECT",
            "The distinct physical form the spell assumes as it bends reality, and its power scaling with greater mastery.",
        );

        let duration = components::select_field(
            "DURATION",
            &self.fields.duration,
            SpellFormMessage::DurationSelected,
        )
        .placeholder("Select a duration");

        let range =
            components::select_field("RANGE", &self.fields.range, SpellFormMessage::RangeSelected)
                .placeholder("Select a range");

        let area =
            components::select_field("AREA", &self.fields.area, SpellFormMessage::AreaSelected)
                .placeholder("Select an area");

        let shape: Element<_> = {
            let kind = components::select_field(
                "SHAPE",
                &self.fields.shape_kind,
                SpellFormMessage::ShapeKindSelected,
            )
            .placeholder("Select a shape");

            let (middle, right) = match &self.fields.shape {
                SpellShapeInput::NoShape => (fill_space(), fill_space()),
                SpellShapeInput::Cone { length } => {
                    let length = components::number_field(Some("LENGTH"), length)
                        .placeholder("Select a length")
                        .on_input(SpellFormMessage::ShapeLengthChanged);

                    (length.into(), fill_space())
                }
                SpellShapeInput::Cube { length } => {
                    let length = components::number_field(Some("LENGTH"), length)
                        .placeholder("Select a length")
                        .on_input(SpellFormMessage::ShapeLengthChanged);

                    (length.into(), fill_space())
                }
                SpellShapeInput::Cylinder { radius, height } => {
                    let radius = components::number_field(Some("RADIUS"), radius)
                        .placeholder("Select a radius")
                        .on_input(SpellFormMessage::ShapeRadiusChanged);
                    let height = components::number_field(Some("HEIGHT"), height)
                        .placeholder("Select a height")
                        .on_input(SpellFormMessage::ShapeHeightChanged);

                    (radius.into(), height.into())
                }
                SpellShapeInput::Line { width, length } => {
                    let width = components::number_field(Some("WIDTH"), width)
                        .placeholder("Select a width")
                        .on_input(SpellFormMessage::ShapeWidthChanged);
                    let length = components::number_field(Some("LENGTH"), length)
                        .placeholder("Select a height")
                        .on_input(SpellFormMessage::ShapeLengthChanged);

                    (width.into(), length.into())
                }
                SpellShapeInput::Sphere { radius } => {
                    let radius = components::number_field(Some("RADIUS"), radius)
                        .placeholder("Select a radius")
                        .width(Length::FillPortion(1))
                        .on_input(SpellFormMessage::ShapeRadiusChanged);

                    (radius.into(), fill_space())
                }
            };

            row![kind, middle, right].spacing(BODY_SPACING).into()
        };

        let description = components::text_area_field(
            "DESCRIPTION",
            &self.fields.description,
            SpellFormMessage::DescriptionChanged,
        )
        .height(300);

        let at_higher_levels = components::text_area_field(
            "AT HIGHER LEVELS",
            &self.fields.at_higher_levels,
            SpellFormMessage::AtHigherLevelsChanged,
        )
        .height(100);

        let row1 = row![duration, range, area]
            .spacing(BODY_SPACING)
            .align_y(Alignment::End);

        let form = column![row1, shape, description, at_higher_levels]
            .align_x(Alignment::Center)
            .spacing(BODY_SPACING);
        let body = components::form::section_body(form);

        row![header, body].into()
    }

    fn narrative(&'a self) -> Element<'a, SpellFormMessage> {
        let header = components::form::section_header(
            "NARRATIVE",
            "Thematical descriptions and illustrations.",
        );

        let flavor_text = components::text_area_field(
            "FLAVOR TEXT",
            &self.fields.flavor_text,
            SpellFormMessage::FlavorTextChanged,
        )
        .placeholder("First there is a bead, tiny and bright, soaring toward you. You watch it, mesmerized, almost thinking it beautiful. Then, the sound vanishes, the air turns to glass, and the world simply burns.")
        .height(200);

        let attribution = components::text_field(Some("ATTRIBUTION"), &self.fields.attribution)
            .placeholder("Sergeant Kaelen, recounting the Siege of Oakhaven")
            .on_input(SpellFormMessage::AttributionChanged);

        let images = components::image_field(&self.fields.images)
            .on_clipboard(SpellFormMessage::ImagePasted)
            .on_remove(SpellFormMessage::ImageRemoved);

        let form = column![flavor_text, attribution, images]
            .align_x(Alignment::Center)
            .spacing(BODY_SPACING);
        let body = components::form::section_body(form);

        row![header, body].into()
    }
}

impl ViewContent for SpellForm {
    type Message = SpellFormMessage;

    type Effect = SpellFormEffect;

    fn title(&self) -> &str {
        match self.mode {
            FormMode::Create => "Create spell",
            FormMode::Edit(_) => "Edit spell",
        }
    }

    fn update(&mut self, message: Self::Message) -> (Task<Self::Message>, Option<Self::Effect>) {
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
            SpellFormMessage::ClassToggled(class) => {
                if let Some(index) = self.fields.classes.iter().position(|c| *c == class) {
                    self.fields.classes.swap_remove(index);
                } else {
                    self.fields.classes.push(class);
                }
            }
            SpellFormMessage::TagChanged(tag) => self.fields.tags.set_value(tag),
            SpellFormMessage::TagSubmitted => self.fields.tags.add_selection(),
            SpellFormMessage::TagRemoved(tag_index) => self.fields.tags.remove_selection(tag_index),
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
            SpellFormMessage::MaterialToggled => {
                self.fields.material = !self.fields.material;

                if self.fields.materials.is_empty() {
                    self.fields.materials.push(SpellMaterialInput::default());
                }
            }
            SpellFormMessage::MaterialChanged(index, material) => {
                if let Some(spell_material) = self.fields.materials.get_mut(index) {
                    spell_material.material.set(material);
                }

                // Add new material if last is non-empty
                if let Some(last) = self.fields.materials.last()
                    && !last.material.value().trim().is_empty()
                {
                    self.fields.materials.push(SpellMaterialInput::default());
                }

                // Remove trailing empty materials, always keep last empty
                let last_non_empty = self.fields.materials.iter().rposition(|m| !m.is_empty());
                let new_len = match last_non_empty {
                    // Keep through last non-empty, plus one trailing empty.
                    //
                    // This is +2 because `index` is the last non-empty material,
                    // so +1 to keep that and +2 to keep the empty behind it.
                    //
                    // Take `.min(current_length)` to ensure following truncate is valid.
                    Some(index) => (index + 2).min(self.fields.materials.len()),
                    // All are empty, keep just one.
                    None => 1,
                };

                self.fields.materials.truncate(new_len);
            }
            SpellFormMessage::MaterialWorthChanged(index, worth) => {
                if let Some(spell_material) = self.fields.materials.get_mut(index) {
                    spell_material.worth.set(worth);
                }
            }
            SpellFormMessage::MaterialConsumed(index) => {
                if let Some(spell_material) = self.fields.materials.get_mut(index) {
                    spell_material.consumed = !spell_material.consumed;
                }
            }
            SpellFormMessage::FlavorTextChanged(action) => {
                self.fields.flavor_text.perform(action);
                self.fields.flavor_text.validate();
            }
            SpellFormMessage::AttributionChanged(quote_source) => {
                self.fields.attribution.set(quote_source);
                self.fields.attribution.validate();
            }
            SpellFormMessage::AreaSelected(area) => self.fields.area.set(area),
            SpellFormMessage::ShapeKindSelected(kind) => {
                self.fields.shape_kind.set(kind);
                self.fields.shape = SpellShapeInput::from(kind);
            }
            SpellFormMessage::ShapeLengthChanged(new_length) => {
                if let SpellShapeInput::Cone { length }
                | SpellShapeInput::Cube { length }
                | SpellShapeInput::Line { length, .. } = &mut self.fields.shape
                {
                    length.set(new_length);
                }
            }
            SpellFormMessage::ShapeRadiusChanged(new_radius) => {
                if let SpellShapeInput::Cylinder { radius, .. }
                | SpellShapeInput::Sphere { radius } = &mut self.fields.shape
                {
                    radius.set(new_radius);
                }
            }
            SpellFormMessage::ShapeHeightChanged(new_height) => {
                if let SpellShapeInput::Cylinder { height, .. } = &mut self.fields.shape {
                    height.set(new_height);
                }
            }
            SpellFormMessage::ShapeWidthChanged(new_width) => {
                if let SpellShapeInput::Line { width, .. } = &mut self.fields.shape {
                    width.set(new_width);
                }
            }
            SpellFormMessage::ImagePasted => {
                let clipboard_result = async {
                    tokio::task::spawn_blocking(components::image_field::clipboard::get_image).await
                };

                let task = Task::perform(clipboard_result, |result| {
                    SpellFormMessage::ImageLoaded(result.unwrap_or(Err(ImageError::Unknown)))
                });

                return (task, None);
            }
            SpellFormMessage::ImageLoaded(Ok(bytes)) => self.fields.images.add(bytes),
            SpellFormMessage::ImageLoaded(Err(err)) => tracing::error!("{err}"),
            SpellFormMessage::ImageRemoved(image_number) => self.fields.images.remove(image_number),
        }

        (Task::none(), None)
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let title = self.title();

        let identity = self.identity_section();

        let casting = self.casting_section();

        let effect = self.effect_section();

        let narrative = self.narrative();

        let view = column![title, identity, casting, effect, narrative]
            .align_x(Alignment::Center)
            .spacing(SECTION_SPACING);

        widget::scrollable(view).into()
    }
}

fn fill_space<'a, Message: 'a>() -> Element<'a, Message> {
    widget::space().width(Length::Fill).into()
}
