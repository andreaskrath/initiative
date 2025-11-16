use components::title;
use gpui::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled, Window,
    div, prelude::FluentBuilder, px,
};
use gpui_component::{
    ActiveTheme, StyledExt,
    button::{Button, ButtonVariant, ButtonVariants, Toggle, ToggleGroup, ToggleVariants},
    form::{field, v_form},
    h_flex,
    input::{Input, InputState},
    scroll::ScrollbarAxis,
    select::{Select, SelectState},
};
use strum::{EnumCount, VariantArray};
use types::{
    Class, FormMode, MagicSchool, SpellArea, SpellCastingTime, SpellDuration, SpellLevel,
    SpellRange, SpellShape,
};

pub struct SpellFormView {
    mode: FormMode,
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

impl SpellFormView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, mode: FormMode) -> Self {
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
            mode,
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
        let basic_information = field()
            .col_span(10)
            .child(title("Basic Spell Information", cx.theme()));

        let name = field()
            .label("Name")
            .required(true)
            .col_span(3)
            .child(Input::new(&self.name));

        let school = field()
            .label("School")
            .required(true)
            .col_span(2)
            .child(Select::new(&self.school).placeholder("Select a magic school"));

        let level = field()
            .label("Level")
            .required(true)
            .col_span(2)
            .child(Select::new(&self.level).placeholder("Select a spell level"));

        let ritual = field().col_span(1).label("").child(
            Toggle::new("ritual")
                .label("Ritual")
                .outline()
                .checked(self.ritual)
                .on_click(cx.listener(|form, checked, _, _| form.ritual = *checked)),
        );

        let concentration = field().col_span(2).label("").child(
            Toggle::new("concentration")
                .label("Concentration")
                .outline()
                .checked(self.concentration)
                .on_click(cx.listener(|form, checked, _, _| form.concentration = *checked)),
        );

        let casting_time = field()
            .label("Casting Time")
            .required(true)
            .col_span(2)
            .child(Select::new(&self.casting_time).placeholder("Select a casting time"));

        let duration = field()
            .label("Duration")
            .required(true)
            .col_span(2)
            .child(Select::new(&self.duration).placeholder("Select a duration"));

        let range = field()
            .label("Range")
            .required(true)
            .col_span(2)
            .child(Select::new(&self.range).placeholder("Select a range"));

        let area = field()
            .label("Area")
            .required(true)
            .col_span(2)
            .child(Select::new(&self.area).placeholder("Select an area"));

        let shape = field().label("Shape").required(true).col_span(2).child(
            Select::new(&self.shape)
                .placeholder("Select a shape")
                .cleanable(true),
        );

        let description = field()
            .col_span(10)
            .label("Description")
            .required(true)
            .child(Input::new(&self.description));

        let at_higher_levels = field()
            .col_span(10)
            .label("At Higher Levels")
            .child(Input::new(&self.at_higher_levels));

        let components = field().col_span(10).child(title("Components", cx.theme()));

        let verbal = field().col_span(1).label("").child(
            Toggle::new("verbal")
                .label("Verbal")
                .outline()
                .checked(self.verbal)
                .on_click(cx.listener(|form, checked, _, _| form.verbal = *checked)),
        );

        let somatic = field().col_span(1).label("").child(
            Toggle::new("somatic")
                .label("Somatic")
                .outline()
                .checked(self.somatic)
                .on_click(cx.listener(|form, checked, _, _| form.somatic = *checked)),
        );

        let material = field().col_span(1).label("").child(
            Toggle::new("material")
                .label("Material")
                .outline()
                .checked(self.material)
                .on_click(cx.listener(|form, checked, _, _| form.material = *checked)),
        );

        let material_consumed = field().col_span(2).when(self.material, |this| {
            this.label("").child(
                Toggle::new("material_consumed")
                    .label("Material Consumed")
                    .outline()
                    .checked(self.material_consumed)
                    .on_click(cx.listener(|form, checked, _, _| form.material_consumed = *checked)),
            )
        });

        let material_description = field().col_span(5).when(self.material, |this| {
            this.label("Materials")
                .required(true) // Not actually required, only if shown
                .child(Input::new(&self.material_description))
        });

        let class_restrictions = field()
            .col_span(10)
            .child(title("Class Restrictions", cx.theme()));

        let classes = field().col_span(10).child(
            ToggleGroup::new("classes")
                .outline()
                .justify_between()
                .w_full()
                .children(Class::VARIANTS.iter().enumerate().map(|(i, class)| {
                    Toggle::new(i)
                        .label(class.to_string())
                        .checked(self.classes[*class])
                }))
                .on_click(cx.listener(|form, states: &Vec<bool>, _, _| {
                    form.classes
                        .iter_mut()
                        .zip(states)
                        .for_each(|(state, checked)| *state = *checked)
                })),
        );

        let submit_button = field().col_span(10).child(
            h_flex().w_full().justify_end().pt_5().child(
                Button::new("submit")
                    .label(self.submit_button_text())
                    .with_variant(ButtonVariant::Primary),
            ),
        );

        div()
            .scrollable(ScrollbarAxis::Vertical)
            .w(px(1200.0))
            .mx_auto()
            .p_3()
            .child(
                v_form()
                    .gap(px(3.0))
                    .columns(10)
                    // Basic Spell Information
                    .child(basic_information)
                    .children([name, school, level, ritual, concentration])
                    .children([casting_time, duration, range, area, shape])
                    .child(description)
                    .child(at_higher_levels)
                    // Componenets
                    .child(components)
                    .children([
                        verbal,
                        somatic,
                        material,
                        material_consumed,
                        material_description,
                    ])
                    // Class Restrictions
                    .child(class_restrictions)
                    .child(classes)
                    // Submit Button
                    .child(submit_button),
            )
    }
}
