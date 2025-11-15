use gpui::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};
use gpui_component::{
    form::{field, v_form},
    input::{Input, InputState},
    select::{Select, SelectState},
};
use strum::VariantArray;
use types::{FormMode, MagicSchool, SpellLevel};

pub struct SpellFormView {
    mode: FormMode,
    name: Entity<InputState>,
    school: Entity<SelectState<Vec<MagicSchool>>>,
    level: Entity<SelectState<Vec<SpellLevel>>>,
}

impl SpellFormView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, mode: FormMode) -> Self {
        let name = cx.new(|cx| InputState::new(window, cx));
        let school =
            cx.new(|cx| SelectState::new(MagicSchool::VARIANTS.to_vec(), None, window, cx));
        let level = cx.new(|cx| SelectState::new(SpellLevel::VARIANTS.to_vec(), None, window, cx));
        let material = cx.new(|cx| InputState::new(window, cx));

        Self {
            mode,
            name,
            school,
            level,
        }
    }
}

impl Render for SpellFormView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let name = field()
            .label("Name")
            .required(true)
            .col_span(4)
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

        div().w(px(1000.0)).mx_auto().p_4().child(
            v_form()
                .gap(px(5.0))
                .columns(8)
                // First row
                .children([name, school, level]),
        )
    }
}
