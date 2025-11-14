use gpui::Global;
use types::View;

pub struct AppState {
    pub view: View,
}

impl AppState {
    pub fn new() -> Self {
        Self { view: View::Root }
    }
}

impl Global for AppState {}
