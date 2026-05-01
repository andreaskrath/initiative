pub mod message;

use crate::view::Viewable;
use crate::view::request::Request;
use components::icon::IconName;
use components::icon::IconSize;
use message::Effect;
use message::Message;
use style::button::ButtonClass;
use style::container::ContainerClass;
use style::layout::BODY_SPACING;
use style::layout::INPUT_PADDING;
use style::layout::LABEL_SPACING;
use style::layout::SECTION_SPACING;
use style::svg::SvgClass;
use types::FormMode;
use widgets::Element;

use iced::Alignment;
use iced::Length;
use iced::Task;
use iced::widget;
use iced::widget::column;
use iced::widget::row;

const ITEM_HEIGHT: u32 = 75;

const CREATIONS_PER_ROW: usize = 4;

const ARCHIVES_PER_ROW: usize = 3;

const CREATIONS: [(&str, IconName, Message); 1] =
    [("New Spell", IconName::WandSparkles, Message::OpenNewSpell)];

const ARCHIVES: [(&str, IconName, Message); 1] =
    [("Spells", IconName::Library, Message::OpenSpells)];

pub struct Dashboard;

impl Dashboard {
    pub fn new() -> Self {
        Self
    }
}

impl Viewable for Dashboard {
    type Message = Message;

    type Effect = Effect;

    fn title(&self) -> &str {
        "Dashboard"
    }

    fn update(&mut self, message: Self::Message) -> (Task<Self::Message>, Option<Self::Effect>) {
        match message {
            Message::OpenSpells => {
                let request = Request::SpellList;
                let effect = Effect::OpenView(request);

                (Task::none(), Some(effect))
            }
            Message::OpenNewSpell => {
                let request = Request::SpellForm {
                    mode: FormMode::Create,
                };
                let effect = Effect::OpenView(request);

                (Task::none(), Some(effect))
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let creations = {
            let header = components::form::section_header(
                "CREATION",
                "Define destiny and the creatures affected by it.",
            );

            let mut buttons = Vec::with_capacity(CREATIONS.len());
            for (text, icon, message) in CREATIONS.into_iter() {
                let button = creation_card(text, icon, message);

                buttons.push(button);
            }

            let rows = components::row::chunked(buttons, CREATIONS_PER_ROW);

            let creations = widget::container(rows)
                .class(ContainerClass::Surface)
                .padding(INPUT_PADDING);

            column![header, creations].spacing(BODY_SPACING)
        };

        let archives = {
            let header = components::form::section_header(
                "ARCHIVE",
                "The ancient records of your currated collection.",
            );

            let mut buttons = Vec::with_capacity(ARCHIVES.len());
            for (text, icon, message) in ARCHIVES.into_iter() {
                let button = archive_card(text, icon, message);

                buttons.push(button);
            }

            let rows = components::row::chunked(buttons, ARCHIVES_PER_ROW);

            let archives = widget::container(rows)
                .class(ContainerClass::Surface)
                .padding(INPUT_PADDING);

            column![header, archives].spacing(BODY_SPACING)
        };

        let chronicles = {
            components::form::section_header(
                "CHRONICLE",
                "Witness the unfolding echoes of past decisions.",
            )
        };

        let column = column![creations, archives]
            .spacing(SECTION_SPACING)
            .width(Length::FillPortion(2));

        row![column, chronicles].spacing(SECTION_SPACING).into()
    }
}

fn creation_card<'a>(text: &'a str, icon: IconName, message: Message) -> Element<'a, Message> {
    let label = components::text::display(text);
    let icon = components::icon(icon)
        .size(IconSize::Large)
        .class(SvgClass::Primary);

    let column = column![
        widget::space().height(Length::Fill),
        icon,
        label,
        widget::space().height(Length::Fill),
    ]
    .align_x(Alignment::Center)
    .spacing(BODY_SPACING)
    .width(Length::Fill);

    widget::button(column)
        .class(ButtonClass::Interaction)
        .width(Length::FillPortion(1))
        .height(ITEM_HEIGHT)
        .on_press(message)
        .into()
}

fn archive_card<'a>(text: &'a str, icon: IconName, message: Message) -> Element<'a, Message> {
    let label = {
        let label = components::text::display(text);
        let entries_text = format!("{} entries", 5);
        let entries = components::text::detail(entries_text);

        column![
            widget::space().height(Length::Fill),
            label,
            entries,
            widget::space().height(Length::Fill),
        ]
        .spacing(LABEL_SPACING)
        .height(Length::Fill)
    };

    let icon = components::icon(icon)
        .size(IconSize::Custom(40))
        .class(SvgClass::Primary);

    let row = row![label, widget::space().width(Length::Fill), icon]
        .align_y(Alignment::Center)
        .padding(INPUT_PADDING);

    widget::button(row)
        .class(ButtonClass::Interaction)
        .width(Length::FillPortion(1))
        .height(ITEM_HEIGHT)
        .on_press(message)
        .into()
}
