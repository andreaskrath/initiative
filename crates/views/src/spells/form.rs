use components::{container, field, title};
use gpui::{
    AppContext, Context, Entity, FocusHandle, IntoElement, ParentElement, Render, SharedString,
    Styled, Subscription, Window, div, prelude::FluentBuilder,
};
use gpui_component::{
    ActiveTheme,
    button::{Button, ButtonVariant, ButtonVariants, Toggle, ToggleGroup, ToggleVariants},
    h_flex,
    input::{Input, InputState},
    select::{Select, SelectState},
};
use strum::{EnumCount, VariantArray};
use types::{
    Class, FormMode, MagicSchool, SpellArea, SpellCastingTime, SpellDuration, SpellLevel,
    SpellRange, SpellShape,
};

struct Form {
    name: Entity<InputState>,
    school: Entity<SelectState<Vec<MagicSchool>>>,
    level: Entity<SelectState<Vec<SpellLevel>>>,
    verbal: bool,
    somatic: bool,
    material: bool,
    material_description: Entity<InputState>,
    material_consumed: bool,
    ritual: bool,
    concentration: bool,
    casting_time: Entity<SelectState<Vec<SpellCastingTime>>>,
    duration: Entity<SelectState<Vec<SpellDuration>>>,
    range: Entity<SelectState<Vec<SpellRange>>>,
    area: Entity<SelectState<Vec<SpellArea>>>,
    shape: Entity<SelectState<Vec<SpellShape>>>,
    description: Entity<InputState>,
    at_higher_levels: Entity<InputState>,
    classes: [bool; Class::COUNT],
}

impl Form {
    fn new(window: &mut Window, cx: &mut Context<SpellFormView>) -> Self {
        let name = cx.new(|cx| InputState::new(window, cx));
        let school =
            cx.new(|cx| SelectState::new(MagicSchool::VARIANTS.to_vec(), None, window, cx));
        let level = cx.new(|cx| SelectState::new(SpellLevel::VARIANTS.to_vec(), None, window, cx));
        let material_description = cx.new(|cx| InputState::new(window, cx));
        let casting_time =
            cx.new(|cx| SelectState::new(SpellCastingTime::VARIANTS.to_vec(), None, window, cx));
        let duration =
            cx.new(|cx| SelectState::new(SpellDuration::VARIANTS.to_vec(), None, window, cx));
        let range = cx.new(|cx| SelectState::new(SpellRange::VARIANTS.to_vec(), None, window, cx));
        let area = cx.new(|cx| SelectState::new(SpellArea::VARIANTS.to_vec(), None, window, cx));
        let shape = cx.new(|cx| SelectState::new(SpellShape::VARIANTS.to_vec(), None, window, cx));
        let description = cx.new(|cx| InputState::new(window, cx).auto_grow(5, 50));
        let at_higher_levels = cx.new(|cx| InputState::new(window, cx).auto_grow(1, 10));

        Self {
            name,
            school,
            level,
            verbal: false,
            somatic: false,
            material: false,
            material_description,
            material_consumed: false,
            ritual: false,
            concentration: false,
            casting_time,
            duration,
            range,
            area,
            shape,
            description,
            at_higher_levels,
            classes: [false; Class::COUNT],
        }
    }

    fn reset(&mut self, window: &mut Window, cx: &mut Context<SpellFormView>) {
        let inputs = [
            &self.name,
            &self.material_description,
            &self.description,
            &self.at_higher_levels,
        ];

        for input in inputs {
            cx.update_entity(input, |entity, cx| entity.set_value("", window, cx));
        }

        self.school
            .update(cx, |school, cx| school.set_selected_index(None, window, cx));

        self.level
            .update(cx, |level, cx| level.set_selected_index(None, window, cx));

        self.casting_time.update(cx, |casting_time, cx| {
            casting_time.set_selected_index(None, window, cx)
        });

        self.duration.update(cx, |duration, cx| {
            duration.set_selected_index(None, window, cx)
        });

        self.range
            .update(cx, |range, cx| range.set_selected_index(None, window, cx));

        self.area
            .update(cx, |area, cx| area.set_selected_index(None, window, cx));

        self.shape
            .update(cx, |shape, cx| shape.set_selected_index(None, window, cx));

        self.verbal = false;
        self.somatic = false;
        self.material = false;
        self.material_consumed = false;
        self.ritual = false;
        self.concentration = false;
        self.classes = [false; Class::COUNT];

        cx.notify();
    }
}

#[derive(Default)]
struct Errors {
    name: Option<SharedString>,
}

pub struct SpellFormView {
    mode: FormMode,
    form: Form,
    errors: Errors,
    focus: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl SpellFormView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, mode: FormMode) -> Self {
        let subscriptions = vec![
            // cx.subscribe_in(&name, window, |form, input, event: &InputEvent, _, cx| {
            //     let value = input.read(cx).value();
            //     if let InputEvent::Blur = event
            //         && value.is_empty()
            //         && form.errors.name.is_none()
            //     {
            //         cx.spawn(async move |form, cx| {
            //             cx.background_executor()
            //                 .timer(Duration::from_millis(100))
            //                 .await;
            //             form.update(cx, |form, cx| {
            //                 form.errors.name = Some("A name must be specified".into());
            //                 cx.notify();
            //             })
            //             .ok();
            //         })
            //         .detach();
            //     }
            // }),
        ];

        Self {
            mode,
            form: Form::new(window, cx),
            errors: Errors::default(),
            focus: cx.focus_handle(),
            _subscriptions: subscriptions,
        }
    }

    fn submit_button_text(&self) -> SharedString {
        match self.mode {
            FormMode::Create => "Create",
            FormMode::Edit(_) => "Save",
        }
        .into()
    }
}

impl Render for SpellFormView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let basic_information = div()
            .col_span(10)
            .child(title("Basic Spell Information", cx.theme()));

        let name = field("Name", Input::new(&self.form.name))
            .required()
            .when_some(self.errors.name.clone(), |this, error| this.error(error))
            .col_span(3);

        let school = field(
            "School",
            Select::new(&self.form.school).placeholder("Select a magic school"),
        )
        .required()
        .col_span(2);

        let level = field(
            "Level",
            Select::new(&self.form.level).placeholder("Select a spell level"),
        )
        .required()
        .col_span(2);

        let ritual = field(
            "",
            Toggle::new("ritual")
                .label("Ritual")
                .outline()
                .checked(self.form.ritual)
                .on_click(cx.listener(|view, checked, _, _| view.form.ritual = *checked)),
        )
        .col_span(1);

        let concentration = field(
            "",
            Toggle::new("concentration")
                .label("Concentration")
                .outline()
                .checked(self.form.concentration)
                .on_click(cx.listener(|view, checked, _, _| view.form.concentration = *checked)),
        )
        .col_span(2);

        let casting_time = field(
            "Casting Time",
            Select::new(&self.form.casting_time).placeholder("Select a casting time"),
        )
        .required()
        .col_span(2);

        let duration = field(
            "Duration",
            Select::new(&self.form.duration).placeholder("Select a duration"),
        )
        .required()
        .col_span(2);

        let range = field(
            "Range",
            Select::new(&self.form.range).placeholder("Select a range"),
        )
        .required()
        .col_span(2);

        let area = field(
            "Area",
            Select::new(&self.form.area).placeholder("Select a range"),
        )
        .required()
        .col_span(2);

        let shape = field(
            "Shape",
            Select::new(&self.form.shape).placeholder("Select a shape"),
        )
        .required()
        .col_span(2);

        let description = field("Description", Input::new(&self.form.description))
            .required()
            // .when_some(self.errors.name.clone(), |this, error| this.error(error))
            .col_span(10);

        let at_higher_levels = field("At Higher Levels", Input::new(&self.form.at_higher_levels))
            .required()
            // .when_some(self.errors.name.clone(), |this, error| this.error(error))
            .col_span(10);

        let components_section = div().col_span(10).child(title("Components", cx.theme()));

        let verbal = field(
            "",
            Toggle::new("verbal")
                .label("Verbal")
                .outline()
                .checked(self.form.verbal)
                .on_click(cx.listener(|view, checked, _, _| view.form.verbal = *checked)),
        )
        .col_span(1);

        let somatic = field(
            "",
            Toggle::new("somatic")
                .label("Somatic")
                .outline()
                .checked(self.form.somatic)
                .on_click(cx.listener(|view, checked, _, _| view.form.somatic = *checked)),
        )
        .col_span(1);

        let material = field(
            "",
            Toggle::new("material")
                .label("Material")
                .outline()
                .checked(self.form.material)
                .on_click(cx.listener(|view, checked, _, _| view.form.material = *checked)),
        )
        .col_span(1);

        let material_consumed = field(
            "",
            Toggle::new("material_consumed")
                .label("Material Consumed")
                .outline()
                .checked(self.form.material_consumed)
                .on_click(
                    cx.listener(|view, checked, _, _| view.form.material_consumed = *checked),
                ),
        )
        .col_span(2);

        let material_description = field("Materials", Input::new(&self.form.material_description))
            .when(self.form.material, |this| this.required())
            .col_span(5);

        let class_restrictions = div()
            .col_span(10)
            .child(title("Class Restrictions", cx.theme()));

        let classes = field(
            "",
            ToggleGroup::new("classes")
                .outline()
                .justify_between()
                .w_full()
                .children(Class::VARIANTS.iter().enumerate().map(|(i, class)| {
                    Toggle::new(i)
                        .label(class.to_string())
                        .checked(self.form.classes[*class])
                }))
                .on_click(cx.listener(|view, states: &Vec<bool>, _, _| {
                    view.form
                        .classes
                        .iter_mut()
                        .zip(states)
                        .for_each(|(state, checked)| *state = *checked)
                })),
        )
        .col_span(10);

        let reset_button = Button::new("reset")
            .label("Reset")
            .with_variant(ButtonVariant::Info)
            .on_click(cx.listener(|view, _, window, cx| {
                view.form.reset(window, cx);
                view.errors = Errors::default();
            }));
        let submit_button = Button::new("submit")
            .label(self.submit_button_text())
            .with_variant(ButtonVariant::Primary);
        let buttons = div().col_span(10).child(
            h_flex()
                .w_full()
                .justify_end()
                .pt_5()
                .gap_2()
                .child(reset_button)
                .child(submit_button),
        );

        container(self.focus.clone())
            .grid()
            .columns(10)
            // Basic Spell Information
            .child(basic_information)
            .child(name)
            .child(school)
            .child(level)
            .child(ritual)
            .child(concentration)
            .child(casting_time)
            .child(duration)
            .child(range)
            .child(area)
            .child(shape)
            .child(description)
            .child(at_higher_levels)
            // Components
            .child(components_section)
            .child(verbal)
            .child(somatic)
            .child(material)
            .child(material_consumed)
            .child(material_description)
            // Class Restrictions
            .child(class_restrictions)
            .child(classes)
            // Buttons
            .child(buttons)
    }
}
