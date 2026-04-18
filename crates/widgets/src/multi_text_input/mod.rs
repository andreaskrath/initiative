use style::text_input::TextInputClass;
use style::theme::Theme;

use iced::Element;
use iced::Event;
use iced::Length;
use iced::Point;
use iced::Rectangle;
use iced::Size;
use iced::Vector;
use iced::advanced::Clipboard;
use iced::advanced::Layout;
use iced::advanced::Shell;
use iced::advanced::Widget;
use iced::advanced::layout::Limits;
use iced::advanced::layout::Node;
use iced::advanced::renderer::Style;
use iced::advanced::text;
use iced::advanced::widget::Operation;
use iced::advanced::widget::Tree;
use iced::advanced::widget::tree::State;
use iced::advanced::widget::tree::Tag;
use iced::mouse::Cursor;
use iced::mouse::Interaction;
use iced::overlay;
use iced::widget;
use iced::widget::TextInput;
use iced::widget::text_input::Value;
use iced_aw::Wrap;
use iced_aw::direction::Horizontal;

const WIDGET_SPACING: f32 = 5.0;

const CHIPS_SPACING: f32 = 5.0;

pub fn multi_text_input<'a, Message, Renderer>(
    placeholder: &'a str,
    value: &'a str,
    values: &'a [String],
) -> MultiTextInput<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: text::Renderer<Font = iced::Font> + 'a,
{
    MultiTextInput::new(placeholder, value, values)
}

pub struct MultiTextInput<'a, Message, Renderer>
where
    Renderer: text::Renderer<Font = iced::Font>,
{
    text_input: TextInput<'a, Message, Theme, Renderer>,
    value: Value,
    chips: Wrap<'a, Message, Horizontal, Theme, Renderer>,
    values: &'a [String],
    width: Length,
}

impl<'a, Message, Renderer> MultiTextInput<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: text::Renderer<Font = iced::Font> + 'a,
{
    pub fn new(placeholder: &'a str, value: &'a str, values: &'a [String]) -> Self {
        let text_input = crate::text_input(placeholder, value);
        let chips = Self::chips(values, None::<fn(usize) -> Message>);

        let value = Value::new(value);

        Self {
            text_input,
            value,
            chips,
            values,
            width: Length::Fill,
        }
    }

    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.text_input = self.text_input.on_input(on_input);
        self
    }

    pub fn on_submit(mut self, on_submit: Message) -> Self {
        self.text_input = self.text_input.on_submit(on_submit);
        self
    }

    pub fn on_remove(mut self, on_remove: impl Fn(usize) -> Message + 'a) -> Self {
        self.chips = Self::chips(self.values, Some(on_remove));
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn class(mut self, class: TextInputClass) -> Self {
        self.text_input = self.text_input.class(class);
        self
    }

    fn chips(
        values: &'a [String],
        on_remove: Option<impl Fn(usize) -> Message + 'a>,
    ) -> Wrap<'a, Message, Horizontal, Theme, Renderer> {
        let mut chips = Vec::with_capacity(values.len());
        for (index, chip) in values.iter().enumerate() {
            let mut button = widget::button(chip.as_str());

            if let Some(on_remove) = &on_remove {
                button = button.on_press(on_remove(index));
            }

            chips.push(button.into());
        }

        Wrap::with_elements(chips)
            .spacing(CHIPS_SPACING)
            .line_spacing(CHIPS_SPACING)
    }

    fn widget_trees(tree: &Tree) -> (&Tree, &Tree) {
        let [text_input_tree, chips_tree] = &tree.children[..] else {
            unreachable!("two widget trees are defined in Widget::children()")
        };

        (text_input_tree, chips_tree)
    }

    fn widget_trees_mut(tree: &mut Tree) -> (&mut Tree, &mut Tree) {
        let [text_input_tree, chips_tree] = &mut tree.children[..] else {
            unreachable!("two widget trees are defined in Widget::children()")
        };

        (text_input_tree, chips_tree)
    }

    fn widget_layouts<'l>(layout: Layout<'l>) -> (Layout<'l>, Layout<'l>) {
        let mut layout_children = layout.children();
        let text_input_layout = layout_children
            .next()
            .expect("TextInput is first child layout");
        let chips_layout = layout_children.next().expect("Wrap is second child layout");

        (text_input_layout, chips_layout)
    }
}

impl<'a, Message, Renderer> Widget<Message, Theme, Renderer>
    for MultiTextInput<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: text::Renderer<Font = iced::Font> + 'a,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let (text_input_tree, chips_tree) = Self::widget_trees_mut(tree);

        // Calculate TextInput layout, and find the height of TextInput
        let text_input_layout =
            self.text_input
                .layout(text_input_tree, renderer, limits, Some(&self.value));
        let text_input_height = text_input_layout.size().height;

        // Calculate the Wrap layout, and offset based on TextInput layout
        let chips_limits = limits.shrink(Size::new(0.0, text_input_height + WIDGET_SPACING));
        let chips_layout = self
            .chips
            .layout(chips_tree, renderer, &chips_limits)
            .move_to(Point::new(0.0, text_input_height + WIDGET_SPACING));

        // Define widget layout based on TextInput and Wrap
        let widget_height = text_input_height + WIDGET_SPACING + chips_layout.size().height;
        let widget_size = Size::new(limits.max().width, widget_height);
        let children = vec![text_input_layout, chips_layout];

        Node::with_children(widget_size, children)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let (text_input_tree, chips_tree) = Self::widget_trees(tree);
        let (text_input_layout, chips_layout) = Self::widget_layouts(layout);

        // Draw TextInput with its own widget tree and layout
        self.text_input.draw(
            text_input_tree,
            renderer,
            theme,
            text_input_layout,
            cursor,
            Some(&self.value),
            viewport,
        );

        // Draw Wrap with its own widget tree and layout
        self.chips.draw(
            chips_tree,
            renderer,
            theme,
            style,
            chips_layout,
            cursor,
            viewport,
        );
    }

    fn tag(&self) -> Tag {
        Tag::stateless()
    }

    fn state(&self) -> State {
        State::None
    }

    fn children(&self) -> Vec<Tree> {
        vec![
            // First element is TextInput tree
            Tree::new(&self.text_input as &dyn Widget<Message, Theme, Renderer>),
            // Second element is Wrap tree
            Tree::new(&self.chips as &dyn Widget<Message, Theme, Renderer>),
        ]
    }

    fn diff(&self, tree: &mut Tree) {
        let (text_input_tree, chips_tree) = Self::widget_trees_mut(tree);

        // Need to reconcile children from tree to widget, instead of other way around.
        // Otherwise, because trees are tagged as being stateless, other widgets trees might be
        // used instead. Calling `diff` form tree, ensures type safety, or they recreate the tree.
        //
        // This is how [`iced::widget::Column`] handles `diff` internally (calling from tree).
        text_input_tree.diff(&self.text_input as &dyn Widget<Message, Theme, Renderer>);
        chips_tree.diff(&self.chips as &dyn Widget<Message, Theme, Renderer>);
    }

    fn operate(
        &mut self,
        _tree: &mut Tree,
        _layout: Layout<'_>,
        _renderer: &Renderer,
        _operation: &mut dyn Operation,
    ) {
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let (text_input_tree, chips_tree) = Self::widget_trees_mut(tree);
        let (text_input_layout, chips_layout) = Self::widget_layouts(layout);

        // Update TextInput with its own widget tree and layout
        self.text_input.update(
            text_input_tree,
            event,
            text_input_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        // Update Wrap with its own widget tree and layout
        self.chips.update(
            chips_tree,
            event,
            chips_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> Interaction {
        let (text_input_tree, chips_tree) = Self::widget_trees(tree);
        let (text_input_layout, chips_layout) = Self::widget_layouts(layout);

        // Call TextInput with its own widget tree and layout
        let text_input_mouse = self.text_input.mouse_interaction(
            text_input_tree,
            text_input_layout,
            cursor,
            viewport,
            renderer,
        );

        // Call Wrap with its own widget tree and layout
        let chips_mouse =
            self.chips
                .mouse_interaction(chips_tree, chips_layout, cursor, viewport, renderer);

        // Use highest priority mouse interaction
        text_input_mouse.max(chips_mouse)
    }

    fn overlay<'b>(
        &'b mut self,
        _tree: &'b mut Tree,
        _layout: Layout<'b>,
        _renderer: &Renderer,
        _viewport: &Rectangle,
        _translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        None
    }
}

impl<'a, Message, Renderer> From<MultiTextInput<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: text::Renderer<Font = iced::Font> + 'a,
{
    fn from(widget: MultiTextInput<'a, Message, Renderer>) -> Self {
        Element::new(widget)
    }
}
