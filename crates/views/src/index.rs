use gpui::{Context, IntoElement, ParentElement, Render, Window, div};

pub struct IndexView;

impl IndexView {
    pub fn new() -> Self {
        Self
    }
}

impl Render for IndexView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().child("Index")
    }
}
