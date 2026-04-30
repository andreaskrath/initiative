mod fields;
mod loader;
pub mod message;

use crate::context::Context;
use crate::status::Status;
use crate::view::View;
use crate::view::spell::form::fields::Fields;
use crate::view::spell::form::fields::SpellMaterialInput;
use crate::view::spell::form::fields::SpellShapeInput;
use crate::view::spell::form::loader::Loader;
use crate::view::spell::form::message::Effect;
use crate::view::spell::form::message::Message;
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
    status: Status<Loader, Fields>,
}

impl<'a> SpellForm {
    pub fn new(mode: FormMode, context: Context) -> (Self, Task<Message>) {
        let (loader, tasks) = Loader::new(context);
        let mapped_tasks = tasks.map(Message::LoadMessage);

        let spell_form = Self {
            mode,
            status: Status::Loading(Box::new(loader)),
        };

        (spell_form, mapped_tasks)
    }

    fn heading(_fields: &'a Fields) -> Element<'a, Message> {
        let title = components::text::view_title("Spell Forge");

        let sub_title = components::text::view_sub_title("Inscribe thy arcane workings");

        column![title, sub_title]
            .width(Fill)
            .align_x(Alignment::Center)
            .into()
    }

    fn identity_section(fields: &'a Fields) -> Element<'a, Message> {
        let header = components::form::section_header(
            "IDENTITY",
            "Fundamental classification and class availability.",
        );

        let name = components::text_field(Some("NAME"), &fields.name)
            .placeholder("Goblin")
            .on_input(Message::NameChanged);
        let school = components::select_field("SCHOOL", &fields.school, Message::SchoolSelected)
            .placeholder("Select a magic school");
        let level = components::select_field("LEVEL", &fields.level, Message::LevelSelected)
            .placeholder("Select a spell level");

        let classes: Element<_> = {
            let elements_selected_text = format!("{} classes selected", fields.classes.len());
            let elements_selected = components::text::detail(elements_selected_text);
            let label = row![
                Label::new("CLASSES").required(false),
                widget::space::horizontal().width(Length::Fill),
                elements_selected
            ];

            let mut cols = Column::new().spacing(BODY_SPACING);

            for row_variants in SPELLCASTING_CLASSES.chunks(3) {
                let row = Row::with_children(row_variants.iter().map(|class| {
                    components::toggle(class.as_ref(), fields.classes.contains(class))
                        .width(Length::Fill)
                        .on_toggle(Message::ClassToggled(*class))
                        .into()
                }))
                .spacing(BODY_SPACING);

                cols = cols.push(row);
            }

            let grid: Element<_> = cols.into();

            column![label, grid].spacing(LABEL_SPACING).into()
        };

        let tags = components::multi_text_field(Some("TAGS"), &fields.tags)
            .placeholder("Write a tag")
            .on_input(Message::TagChanged)
            .on_submit(Message::TagSubmitted)
            .on_remove(Message::TagRemoved);

        let classification = row![school, level].spacing(BODY_SPACING);
        let form = column![name, classification, classes, tags].spacing(BODY_SPACING);
        let body = components::form::section_body(form);

        row![header, body].into()
    }

    fn casting_section(fields: &'a Fields) -> Element<'a, Message> {
        let header = components::form::section_header(
            "CASTING",
            "The mechanics and requirements of casting the spell.",
        );

        let casting_time = components::select_field(
            "CASTING TIME",
            &fields.casting_time,
            Message::CastingTimeSelected,
        )
        .placeholder("Select a casting time");

        let properties: Element<_> = {
            let label = Label::new("PROPERTIES");
            let ritual = components::toggle("Ritual", fields.ritual)
                .width(Fill)
                .on_toggle(Message::RitualToggled);
            let concentration = components::toggle("Concentration", fields.concentration)
                .width(Fill)
                .on_toggle(Message::ConcentrationToggled);
            let toggle_row = row![ritual, concentration].spacing(BODY_SPACING);

            column![label, toggle_row].spacing(LABEL_SPACING).into()
        };

        let components: Element<_> = {
            let label = Label::new("COMPONENTS");
            let verbal = components::toggle("Verbal", fields.verbal)
                .width(Fill)
                .on_toggle(Message::VerbalToggled);
            let somatic = components::toggle("Somatic", fields.somatic)
                .width(Fill)
                .on_toggle(Message::SomaticToggled);
            let material = components::toggle("Material", fields.material)
                .width(Fill)
                .on_toggle(Message::MaterialToggled);
            let components_row = row![verbal, somatic, material].spacing(BODY_SPACING);

            column![label, components_row].spacing(LABEL_SPACING).into()
        };

        let materials: Element<_> = {
            let mut columns = Column::with_capacity(fields.materials.len()).spacing(BODY_SPACING);

            if fields.material {
                for (index, spell_material) in fields.materials.iter().enumerate() {
                    let material_label = if index == 0 { Some("MATERIAL") } else { None };
                    let worth_label = if index == 0 { Some("WORTH") } else { None };
                    let material = components::text_field(material_label, &spell_material.material)
                        .placeholder("A bat wing")
                        .on_input(move |new_material| {
                            Message::MaterialChanged(index, new_material)
                        });
                    let worth = components::text_field(worth_label, &spell_material.worth)
                        .placeholder("100 gp")
                        .on_input(move |new_worth| Message::MaterialWorthChanged(index, new_worth));
                    let consumed = components::toggle("Consumed", spell_material.consumed)
                        .on_toggle(Message::MaterialConsumed(index));
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

    fn effect_section(fields: &'a Fields) -> Element<'a, Message> {
        let header = components::form::section_header(
            "EFFECT",
            "The distinct physical form the spell assumes as it bends reality, and its power scaling with greater mastery.",
        );

        let duration =
            components::select_field("DURATION", &fields.duration, Message::DurationSelected)
                .placeholder("Select a duration");

        let range = components::select_field("RANGE", &fields.range, Message::RangeSelected)
            .placeholder("Select a range");

        let area = components::select_field("AREA", &fields.area, Message::AreaSelected)
            .placeholder("Select an area");

        let shape: Element<_> = {
            let kind =
                components::select_field("SHAPE", &fields.shape_kind, Message::ShapeKindSelected)
                    .placeholder("Select a shape");

            let (middle, right) = match &fields.shape {
                SpellShapeInput::NoShape => (fill_space(), fill_space()),
                SpellShapeInput::Cone { length } => {
                    let length = components::number_field(Some("LENGTH"), length)
                        .placeholder("Select a length")
                        .on_input(Message::ShapeLengthChanged);

                    (length.into(), fill_space())
                }
                SpellShapeInput::Cube { length } => {
                    let length = components::number_field(Some("LENGTH"), length)
                        .placeholder("Select a length")
                        .on_input(Message::ShapeLengthChanged);

                    (length.into(), fill_space())
                }
                SpellShapeInput::Cylinder { radius, height } => {
                    let radius = components::number_field(Some("RADIUS"), radius)
                        .placeholder("Select a radius")
                        .on_input(Message::ShapeRadiusChanged);
                    let height = components::number_field(Some("HEIGHT"), height)
                        .placeholder("Select a height")
                        .on_input(Message::ShapeHeightChanged);

                    (radius.into(), height.into())
                }
                SpellShapeInput::Line { width, length } => {
                    let width = components::number_field(Some("WIDTH"), width)
                        .placeholder("Select a width")
                        .on_input(Message::ShapeWidthChanged);
                    let length = components::number_field(Some("LENGTH"), length)
                        .placeholder("Select a height")
                        .on_input(Message::ShapeLengthChanged);

                    (width.into(), length.into())
                }
                SpellShapeInput::Sphere { radius } => {
                    let radius = components::number_field(Some("RADIUS"), radius)
                        .placeholder("Select a radius")
                        .width(Length::FillPortion(1))
                        .on_input(Message::ShapeRadiusChanged);

                    (radius.into(), fill_space())
                }
            };

            row![kind, middle, right].spacing(BODY_SPACING).into()
        };

        let description = components::text_area_field(
            "DESCRIPTION",
            &fields.description,
            Message::DescriptionChanged,
        )
        .height(300);

        let at_higher_levels = components::text_area_field(
            "AT HIGHER LEVELS",
            &fields.at_higher_levels,
            Message::AtHigherLevelsChanged,
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

    fn narrative(fields: &'a Fields) -> Element<'a, Message> {
        let header = components::form::section_header(
            "NARRATIVE",
            "Thematical descriptions and illustrations.",
        );

        let flavor_text = components::text_area_field(
            "FLAVOR TEXT",
            &fields.flavor_text,
            Message::FlavorTextChanged,
        )
        .placeholder("First there is a bead, tiny and bright, soaring toward you. You watch it, mesmerized, almost thinking it beautiful. Then, the sound vanishes, the air turns to glass, and the world simply burns.")
        .height(200);

        let attribution = components::text_field(Some("ATTRIBUTION"), &fields.attribution)
            .placeholder("Sergeant Kaelen, recounting the Siege of Oakhaven")
            .on_input(Message::AttributionChanged);

        let images = components::image_field(&fields.images)
            .on_clipboard(Message::ImagePasted)
            .on_file_picker(Message::ImagePickerOpened)
            .on_remove(Message::ImageRemoved);

        let form = column![flavor_text, attribution, images]
            .align_x(Alignment::Center)
            .spacing(BODY_SPACING);
        let body = components::form::section_body(form);

        row![header, body].into()
    }
}

impl View for SpellForm {
    type Message = Message;

    type Effect = Effect;

    fn title(&self) -> &str {
        match self.mode {
            FormMode::Create => "Create spell",
            FormMode::Edit(_) => "Edit spell",
        }
    }

    fn update(&mut self, message: Self::Message) -> (Task<Self::Message>, Option<Self::Effect>) {
        match (&mut self.status, message) {
            (Status::Loading(loader), Message::LoadMessage(load_message)) => {
                loader.update(load_message);

                if let Some(err) = loader.error.take() {
                    tracing::error!("failed to load spell form: {:?}", err);

                    return (Task::none(), Some(Effect::LoadFailed(err)));
                }

                // If the loader is done, we construct fields from fetched data (in the current loader state).
                if loader.is_done() {
                    match Fields::from_loader(loader) {
                        Some(fields) => self.status = Status::Ready(Box::new(fields)),
                        None => {
                            tracing::error!("failed to convert loader to fields");
                        }
                    }
                }

                return (Task::none(), None);
            }
            (Status::Ready(fields), Message::NameChanged(name)) => {
                fields.name.set(name);
                fields.name.validate();
            }
            (Status::Ready(fields), Message::SchoolSelected(school)) => {
                fields.school.set(school);
            }
            (Status::Ready(fields), Message::LevelSelected(level)) => {
                fields.level.set(level);
            }
            (Status::Ready(fields), Message::ClassToggled(class)) => {
                if let Some(index) = fields.classes.iter().position(|c| *c == class) {
                    fields.classes.swap_remove(index);
                } else {
                    fields.classes.push(class);
                }
            }
            (Status::Ready(fields), Message::TagChanged(tag)) => fields.tags.set_value(tag),
            (Status::Ready(fields), Message::TagSubmitted) => fields.tags.add_selection(),
            (Status::Ready(fields), Message::TagRemoved(tag_index)) => {
                fields.tags.remove_selection(tag_index)
            }
            (Status::Ready(fields), Message::CastingTimeSelected(casting_time)) => {
                fields.casting_time.set(casting_time);
            }
            (Status::Ready(fields), Message::RitualToggled) => fields.ritual = !fields.ritual,
            (Status::Ready(fields), Message::ConcentrationToggled) => {
                fields.concentration = !fields.concentration;
            }
            (Status::Ready(fields), Message::VerbalToggled) => fields.verbal = !fields.verbal,
            (Status::Ready(fields), Message::SomaticToggled) => fields.somatic = !fields.somatic,
            (Status::Ready(fields), Message::MaterialToggled) => {
                fields.material = !fields.material;

                if fields.materials.is_empty() {
                    fields.materials.push(SpellMaterialInput::default());
                }
            }
            (Status::Ready(fields), Message::MaterialChanged(index, material)) => {
                if let Some(spell_material) = fields.materials.get_mut(index) {
                    spell_material.material.set(material);
                }

                // Add new material if last is non-empty
                if let Some(last) = fields.materials.last()
                    && !last.material.value().trim().is_empty()
                {
                    fields.materials.push(SpellMaterialInput::default());
                }

                // Remove trailing empty materials, always keep last empty
                let last_non_empty = fields.materials.iter().rposition(|m| !m.is_empty());
                let new_len = match last_non_empty {
                    // Keep through last non-empty, plus one trailing empty.
                    //
                    // This is +2 because `index` is the last non-empty material,
                    // so +1 to keep that and +2 to keep the empty behind it.
                    //
                    // Take `.min(current_length)` to ensure following truncate is valid.
                    Some(index) => (index + 2).min(fields.materials.len()),
                    // All are empty, keep just one.
                    None => 1,
                };

                fields.materials.truncate(new_len);
            }
            (Status::Ready(fields), Message::MaterialWorthChanged(index, worth)) => {
                if let Some(spell_material) = fields.materials.get_mut(index) {
                    spell_material.worth.set(worth);
                }
            }
            (Status::Ready(fields), Message::MaterialConsumed(index)) => {
                if let Some(spell_material) = fields.materials.get_mut(index) {
                    spell_material.consumed = !spell_material.consumed;
                }
            }
            (Status::Ready(fields), Message::DurationSelected(duration)) => {
                fields.duration.set(duration);
            }
            (Status::Ready(fields), Message::RangeSelected(range)) => {
                fields.range.set(range);
            }
            (Status::Ready(fields), Message::AreaSelected(area)) => fields.area.set(area),
            (Status::Ready(fields), Message::ShapeKindSelected(kind)) => {
                fields.shape_kind.set(kind);
                fields.shape = SpellShapeInput::from(kind);
            }
            (Status::Ready(fields), Message::ShapeLengthChanged(new_length)) => {
                if let SpellShapeInput::Cone { length }
                | SpellShapeInput::Cube { length }
                | SpellShapeInput::Line { length, .. } = &mut fields.shape
                {
                    length.set(new_length);
                }
            }
            (Status::Ready(fields), Message::ShapeRadiusChanged(new_radius)) => {
                if let SpellShapeInput::Cylinder { radius, .. }
                | SpellShapeInput::Sphere { radius } = &mut fields.shape
                {
                    radius.set(new_radius);
                }
            }
            (Status::Ready(fields), Message::ShapeHeightChanged(new_height)) => {
                if let SpellShapeInput::Cylinder { height, .. } = &mut fields.shape {
                    height.set(new_height);
                }
            }
            (Status::Ready(fields), Message::ShapeWidthChanged(new_width)) => {
                if let SpellShapeInput::Line { width, .. } = &mut fields.shape {
                    width.set(new_width);
                }
            }
            (Status::Ready(fields), Message::DescriptionChanged(action)) => {
                if fields.description.perform(action) {
                    fields.description.validate();
                }
            }
            (Status::Ready(fields), Message::AtHigherLevelsChanged(action)) => {
                fields.at_higher_levels.perform(action);
            }
            (Status::Ready(fields), Message::FlavorTextChanged(action)) => {
                fields.flavor_text.perform(action);
                fields.flavor_text.validate();
            }
            (Status::Ready(fields), Message::AttributionChanged(quote_source)) => {
                fields.attribution.set(quote_source);
                fields.attribution.validate();
            }
            (Status::Ready(_), Message::ImagePasted) => {
                let task = Task::perform(
                    components::image_field::clipboard::get_image(),
                    Message::ImageLoaded,
                );

                return (task, None);
            }
            (Status::Ready(fields), Message::ImageLoaded(Ok(bytes))) => fields.images.add(bytes),
            (Status::Ready(_), Message::ImageLoaded(Err(err))) => {
                tracing::error!("{err}")
            }
            (Status::Ready(fields), Message::ImageRemoved(image_number)) => {
                fields.images.remove(image_number)
            }
            (Status::Ready(_), Message::ImagePickerOpened) => {
                let task = Task::perform(
                    components::image_field::file::open_image_picker(),
                    Message::ImageFileSelected,
                );

                return (task, None);
            }
            (Status::Ready(_), Message::ImageFileSelected(Some(path))) => {
                let task = Task::perform(
                    components::image_field::file::load_image(path),
                    Message::ImageFileLoaded,
                );

                return (task, None);
            }
            (Status::Ready(_), Message::ImageFileSelected(None)) => {
                tracing::error!("selected image file has no path");
            }
            (Status::Ready(fields), Message::ImageFileLoaded(Ok(bytes))) => {
                fields.images.add(bytes)
            }
            (Status::Ready(_), Message::ImageFileLoaded(Err(err))) => {
                tracing::error!("{err}")
            }
            (_invalid_state, invalid_message) => {
                tracing::error!("invalid message: {invalid_message:?}");
            }
        }

        (Task::none(), None)
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match &self.status {
            Status::Loading(loader) => {
                let start = 0.0;
                let end = loader.total as f32;
                let current = loader.progress as f32;

                widget::progress_bar(start..=end, current).into()
            }
            Status::Ready(fields) => {
                let heading = Self::heading(fields);

                let identity = Self::identity_section(fields);

                let casting = Self::casting_section(fields);

                let effect = Self::effect_section(fields);

                let narrative = Self::narrative(fields);

                let view = column![heading, identity, casting, effect, narrative]
                    .align_x(Alignment::Center)
                    .spacing(SECTION_SPACING);

                widget::scrollable(view).into()
            }
        }
    }
}

fn fill_space<'a, Message: 'a>() -> Element<'a, Message> {
    widget::space().width(Length::Fill).into()
}
