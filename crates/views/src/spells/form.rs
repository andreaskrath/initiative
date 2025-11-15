use gpui::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};
use gpui_component::{
    dropdown::{Dropdown, DropdownState},
    form::{form_field, v_form},
    input::{InputState, TextInput},
};
use strum::VariantArray;
use types::{FormMode, MagicSchool, SpellLevel};

pub struct SpellFormView {
    mode: FormMode,
    name: Entity<InputState>,
    school: Entity<DropdownState<Vec<MagicSchool>>>,
    level: Entity<DropdownState<Vec<SpellLevel>>>,
}

impl SpellFormView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, mode: FormMode) -> Self {
        let name = cx.new(|cx| InputState::new(window, cx));
        let school =
            cx.new(|cx| DropdownState::new(MagicSchool::VARIANTS.to_vec(), None, window, cx));
        let level =
            cx.new(|cx| DropdownState::new(SpellLevel::VARIANTS.to_vec(), None, window, cx));

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
        let name = form_field()
            .label("Name")
            .required(true)
            .child(TextInput::new(&self.name));

        let school = form_field()
            .label("School")
            .required(true)
            .child(Dropdown::new(&self.school).placeholder("Select a magic school"));

        let level = form_field()
            .label("Level")
            .required(true)
            .child(Dropdown::new(&self.level).placeholder("Select a spell level"));

        div().w(px(1000.0)).mx_auto().p_4().child(
            v_form()
                .gap(px(5.0))
                .column(3)
                .child(name)
                .child(school)
                .child(level),
        )
    }
}
