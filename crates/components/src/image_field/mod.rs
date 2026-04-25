pub mod clipboard;
pub mod error;
pub mod file;
pub mod state;

use crate::icon::IconName;
use crate::icon::IconSize;
use crate::image_field::state::ImageFieldState;
use crate::label::Label;
use style::button::ButtonClass;
use style::container::ContainerClass;
use style::layout::BODY_SPACING;
use style::layout::INPUT_PADDING;
use style::svg::SvgClass;
use widgets::Element;

use iced::Alignment;
use iced::ContentFit;
use iced::Length;
use iced::widget::Column;
use iced::widget::Row;
use iced::widget::column;
use iced::widget::row;
use iced::widget::stack;

const ITEMS_PER_ROW: usize = 3;

const IMAGE_HEIGHT: u32 = 200;

pub fn image_field<'a, Message>(state: &'a ImageFieldState) -> ImageField<'a, Message> {
    ImageField::new(state)
}

pub struct ImageField<'a, Message> {
    state: &'a ImageFieldState,
    on_clipboard: Option<Message>,
    on_file_picker: Option<Message>,
    on_remove: Option<Box<dyn Fn(usize) -> Message + 'a>>,
}

impl<'a, Message> ImageField<'a, Message> {
    pub fn new(state: &'a ImageFieldState) -> Self {
        Self {
            state,
            on_clipboard: None,
            on_file_picker: None,
            on_remove: None,
        }
    }

    pub fn on_clipboard(mut self, on_clipboard: Message) -> Self {
        self.on_clipboard = Some(on_clipboard);
        self
    }

    pub fn on_file_picker(mut self, on_file_picker: Message) -> Self {
        self.on_file_picker = Some(on_file_picker);
        self
    }

    pub fn on_remove(mut self, on_remove: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_remove = Some(Box::new(on_remove));
        self
    }
}

impl<'a, Message> From<ImageField<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(widget: ImageField<Message>) -> Self {
        let mut number_of_items = widget.state.images.len();

        // Add place for button to add more if not at limit.
        if !widget.state.at_limit() {
            number_of_items += 1
        }

        let padding = calculate_padding(number_of_items, ITEMS_PER_ROW);
        number_of_items += padding;

        let mut items: Vec<Element<_>> = Vec::with_capacity(number_of_items);

        // Add all images.
        for (index, image) in widget.state.images.iter().enumerate() {
            let handle = image.handle.clone();
            let image = iced::widget::image(handle)
                .content_fit(ContentFit::Cover)
                .width(Length::FillPortion(1))
                .height(IMAGE_HEIGHT)
                .border_radius(2);

            let on_remove_maybe = widget.on_remove.as_deref().map(|m| m(index));
            let icon = crate::icon(IconName::Close)
                .class(SvgClass::Danger)
                .size(IconSize::Large);
            let close_button = iced::widget::button(icon)
                .on_press_maybe(on_remove_maybe)
                .class(ButtonClass::Overlay)
                .width(Length::Shrink)
                .height(Length::Shrink);

            let overlay = iced::widget::container(close_button)
                .class(ContainerClass::Ghost)
                .padding(INPUT_PADDING)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_y(Alignment::Start)
                .align_x(Alignment::End);

            let layers = stack![image, overlay];
            items.push(layers.into());
        }

        // Add button for new images.
        if !widget.state.at_limit() {
            let browse_files_button = {
                let icon = crate::icon(IconName::Directory).class(SvgClass::Text);
                let text = crate::text::display("Browse files");
                let label = row![icon, text]
                    .spacing(BODY_SPACING)
                    .width(Length::Fill)
                    .align_y(Alignment::Center);
                iced::widget::button(label)
                    .on_press_maybe(widget.on_file_picker)
                    .width(Length::Shrink)
            };

            let divider = crate::text::display("or");

            let clipboard_button = {
                let icon = crate::icon(IconName::Clipboard).class(SvgClass::Text);
                let text = crate::text::display("Paste from Clipboard");
                let label = row![icon, text]
                    .spacing(BODY_SPACING)
                    .width(Length::Fill)
                    .align_y(Alignment::Center);
                iced::widget::button(label)
                    .on_press_maybe(widget.on_clipboard)
                    .width(Length::Shrink)
            };

            let content = column![browse_files_button, divider, clipboard_button]
                .align_x(Alignment::Center)
                .spacing(BODY_SPACING);

            let container = iced::widget::container(content)
                .class(ContainerClass::Interaction)
                .align_y(Alignment::Center)
                .align_x(Alignment::Center)
                .width(Length::FillPortion(1))
                .height(IMAGE_HEIGHT);
            items.push(container.into());
        }

        // Add padding.
        for _ in 0..padding {
            let space = iced::widget::space()
                .width(Length::FillPortion(1))
                .height(IMAGE_HEIGHT)
                .into();
            items.push(space);
        }

        let mut column =
            Column::with_capacity(number_of_items % ITEMS_PER_ROW).spacing(BODY_SPACING);
        while !items.is_empty() {
            let row_items = items.drain(..ITEMS_PER_ROW);
            let mut row = Row::with_capacity(ITEMS_PER_ROW).spacing(BODY_SPACING);
            for row_item in row_items {
                row = row.push(row_item);
            }

            column = column.push(row);
        }

        column![Label::new("IMAGES"), column].into()
    }
}

/// Calculate the number of padding elements necessary for `current` to become a multiple of `target`.
fn calculate_padding(current: usize, target: usize) -> usize {
    let remainder = current % target;

    // This branch exists for when `current` is already a multiple of `target`.
    //
    // For example:
    // - `current` = 10
    // - `target` = 5
    // - `remainder` = 0
    //
    // Now `target` - `remainder` ends up just being `target` and over-padding.
    if remainder == 0 {
        0
    } else {
        target - remainder
    }
}
