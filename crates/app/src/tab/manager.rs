use crate::tab::Tab;
use crate::tab::TabId;
use crate::tab::TabManagerEffect;
use crate::tab::TabManagerMessage;
use crate::tab::TabMessage;
use crate::tab::bar::tab_bar;
use crate::view::View;
use crate::view::dashboard::Dashboard;
use crate::view::request::Request;
use crate::view::spell::form::SpellForm;
use crate::view::spell::list::SpellList;
use crate::view::spell::list::message::SpellListEffect;
use widgets::Element;

use iced::Alignment;
use iced::Length::Fill;
use iced::Padding;
use iced::Task;
use iced::widget;
use iced::widget::Space;
use iced::widget::column;
use tracing::debug;
use tracing::error;

/// The maximum width a view is allowed to take up.
const VIEW_WIDTH: f32 = 1200.0;

pub struct TabManager {
    tabs: Vec<(TabId, Tab)>,
    active: TabId,
}

impl Default for TabManager {
    fn default() -> Self {
        let mut tabs = Vec::with_capacity(1);
        let id = TabId::unique();
        let initial_tab = Tab::Dashboard(Dashboard::new());
        tabs.push((id, initial_tab));

        Self { tabs, active: id }
    }
}

impl TabManager {
    pub fn update(
        &mut self,
        message: TabManagerMessage,
    ) -> (Task<TabManagerMessage>, Option<TabManagerEffect>) {
        match message {
            TabManagerMessage::TabUpdated(tab_id, tab_message) => {
                return self.update_tab(tab_id, tab_message);
            }
            TabManagerMessage::OpenTab(request) => {
                return self.handle_request(request);
            }
            TabManagerMessage::CloseTab(close_id) => {
                self.tabs.retain(|(tab_id, _)| *tab_id != close_id);

                // TODO: Never redirect to Dashboard (TabId(0)), unless it is the last tab open
                // Only update active if it is the tab being closed
                if self.active == close_id {
                    debug!("closing active tab");

                    let (closest_id, _) = self
                        .tabs
                        .iter()
                        .min_by_key(|(tab_id, _)| tab_id.difference(close_id))
                        .expect("the dashboard cannot be closed, and always exists");

                    self.active = *closest_id;
                }
            }
            TabManagerMessage::FocusTab(tab_id) => self.active = tab_id,
        }

        (Task::none(), None)
    }

    pub fn view(&self) -> Element<'_, TabManagerMessage> {
        let tab_bar = tab_bar(&self.tabs, self.active);

        let divider = widget::rule::horizontal(1);

        let Some(tab) = self.get(self.active) else {
            error!("could not find tab '{:?}'", self.active);

            return Space::new().into();
        };

        let view = match tab {
            Tab::Dashboard(dashboard) => dashboard.view().map(TabMessage::Dashboard),
            Tab::SpellForm(spell_form) => spell_form.view().map(TabMessage::SpellForm),
            Tab::SpellList(spell_list) => spell_list.view().map(TabMessage::SpellList),
        }
        .map(|tab_message| TabManagerMessage::TabUpdated(self.active, tab_message));

        let constrained_content = widget::container(view)
            .max_width(VIEW_WIDTH)
            .padding(Padding::new(0.0).vertical(10.0));

        let centering_container = widget::container(constrained_content).center_x(Fill);

        let scrollable_view = widget::scrollable(centering_container)
            .width(Fill)
            .height(Fill);

        column![tab_bar, divider, scrollable_view]
            .align_x(Alignment::Center)
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn update_tab(
        &mut self,
        tab_id: TabId,
        message: TabMessage,
    ) -> (Task<TabManagerMessage>, Option<TabManagerEffect>) {
        let Some(tab) = self.get_mut(tab_id) else {
            error!("could not find tab with id '{tab_id:?}'");

            return (Task::none(), None);
        };

        match message {
            TabMessage::Dashboard(dashboard_message) => {
                let Tab::Dashboard(dashboard) = tab else {
                    error!(
                        "tab with id '{tab_id:?}' does not match message of type '{dashboard_message:?}'"
                    );

                    return (Task::none(), None);
                };

                let (dashboard_task, maybe_effect) = dashboard.update(dashboard_message);

                let task = wrap_message(dashboard_task, tab_id, TabMessage::Dashboard);

                let mut effect = None;
                if let Some(dashboard_effect) = maybe_effect {
                    match dashboard_effect {}
                }

                (task, effect)
            }
            TabMessage::SpellForm(spell_form_message) => {
                let Tab::SpellForm(spell_form) = tab else {
                    error!(
                        "tab with id '{tab_id:?}' does not match message of type '{spell_form_message:?}'"
                    );

                    return (Task::none(), None);
                };

                let (spell_form_task, maybe_effect) = spell_form.update(spell_form_message);

                let task = wrap_message(spell_form_task, tab_id, TabMessage::SpellForm);

                let mut effect = None;
                if let Some(spell_form_effect) = maybe_effect {
                    match spell_form_effect {}
                }

                (task, effect)
            }
            TabMessage::SpellList(spell_list_message) => {
                let Tab::SpellList(spell_list) = tab else {
                    error!(
                        "tab with id '{tab_id:?}' does not match message of type '{spell_list_message:?}'"
                    );

                    return (Task::none(), None);
                };

                let (spell_list_task, maybe_effect) = spell_list.update(spell_list_message);

                let mut tasks = Vec::with_capacity(2);
                tasks.push(wrap_message(spell_list_task, tab_id, TabMessage::SpellList));

                let mut effect = None;
                if let Some(spell_list_effect) = maybe_effect {
                    match spell_list_effect {
                        SpellListEffect::OpenSpellForm { mode } => {
                            let task =
                                Task::done(TabManagerMessage::OpenTab(Request::SpellForm { mode }));
                            tasks.push(task);
                        }
                    }
                }

                (Task::batch(tasks), effect)
            }
        }
    }

    fn get(&self, id: TabId) -> Option<&Tab> {
        self.tabs
            .iter()
            .find(|(tab_id, _)| *tab_id == id)
            .map(|(_, tab)| tab)
    }

    fn get_mut(&mut self, id: TabId) -> Option<&mut Tab> {
        self.tabs
            .iter_mut()
            .find(|(tab_id, _)| *tab_id == id)
            .map(|(_, tab)| tab)
    }

    fn handle_request(
        &mut self,
        request: Request,
    ) -> (Task<TabManagerMessage>, Option<TabManagerEffect>) {
        debug!("handling tab request: {request:?}");

        match request {
            Request::SpellForm { mode } => {
                let id = TabId::unique();
                let (spell_form, task) = SpellForm::new(mode);
                let tab = Tab::SpellForm(Box::new(spell_form));
                self.tabs.push((id, tab));
                self.active = id;

                let mapped_task =
                    task.map(move |m| TabManagerMessage::TabUpdated(id, TabMessage::SpellForm(m)));

                return (mapped_task, None);
            }
            Request::SpellList => {
                // Check if index already exists
                let Some(tab_id) = self.tab_exists(|tab| matches!(tab, Tab::SpellList(_))) else {
                    let id = TabId::unique();
                    let new_tab = Tab::SpellList(Box::new(SpellList::new()));
                    self.tabs.push((id, new_tab));
                    self.active = id;

                    return (Task::none(), None);
                };

                self.active = tab_id;
            }
        }

        (Task::none(), None)
    }

    /// Returns a reference to a `Tab` if it exists.
    fn tab_exists(&self, predicate: impl Fn(&Tab) -> bool) -> Option<TabId> {
        self.tabs
            .iter()
            .find(|(_, tab)| predicate(tab))
            .map(|(tab_id, _)| *tab_id)
    }
}

fn wrap_message<M>(
    child_task: Task<M>,
    tab_id: TabId,
    wrapper: impl Fn(M) -> TabMessage + Send + Sync + 'static,
) -> Task<TabManagerMessage>
where
    M: Send + Sync + 'static,
{
    child_task.map(move |message| TabManagerMessage::TabUpdated(tab_id, wrapper(message)))
}
