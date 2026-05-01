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

        // Reserving a bit too much, but it doesn't really matter.
        let mut items: Vec<Element<_>> = Vec::with_capacity(number_of_items + ITEMS_PER_ROW);

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

        let images = crate::row::chunked(items, ITEMS_PER_ROW);

        column![Label::new("IMAGES"), images].into()
    }
}
