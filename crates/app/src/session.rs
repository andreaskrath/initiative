use crate::context::Context;
use crate::message::Message;
use crate::view::View;
use crate::view::ViewId;
use crate::view::ViewMessage;
use crate::view::Viewable;
use crate::view::dashboard::Dashboard;
use crate::view::dashboard::message::Effect as DashboardEffect;
use crate::view::request::Request;
use crate::view::spell::form::SpellForm;
use crate::view::spell::form::message::Effect as SpellFormEffect;
use crate::view::spell::list::SpellList;
use crate::view::spell::list::message::Effect as SpellListEffect;
use components::icon::IconName;
use iced::widget::Row;
use iced::widget::scrollable::Direction;
use iced::widget::scrollable::Scrollbar;
use storage::repositories::Repository;
use style::container::ContainerClass;
use style::svg::SvgClass;
use widgets::Element;

use iced::Alignment;
use iced::Length;
use iced::Padding;
use iced::Task;
use iced::widget;
use iced::widget::Space;
use iced::widget::column;
use iced::widget::row;

/// The width a view takes up.
const VIEW_WIDTH: f32 = 1200.0;

/// The height of the overview bar.
const OVERVIEW_BAR_HEIGHT: u32 = 30;

/// The width of each element in the overview bar.
const OVERVIEW_ELEMENT_WIDTH: u32 = 200;

pub struct Session {
    context: Context,
    active_view: ViewId,
    dashboard_id: ViewId,
    views: Vec<(ViewId, View)>,
}

impl Session {
    pub fn new(repository: impl Repository) -> Self {
        let id = ViewId::unique();
        let dashboard = View::Dashboard(Dashboard::new());
        let views = vec![(id, dashboard)];
        let active_view = id;
        let context = Context::new(repository);

        Self {
            context,
            active_view,
            dashboard_id: id,
            views,
        }
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn active_view(&self) -> Element<'_, Message> {
        let overview = self.overview(&self.views, self.active_view);

        let divider = iced::widget::rule::horizontal(1);

        let Some(active_view) = self.view(self.active_view) else {
            tracing::error!("could not find view '{:?}'", self.active_view);

            return Space::new().into();
        };

        let view = match active_view {
            View::Dashboard(dashboard) => dashboard.view().map(ViewMessage::Dashboard),
            View::SpellForm(spell_form) => spell_form.view().map(ViewMessage::SpellForm),
            View::SpellList(spell_list) => spell_list.view().map(ViewMessage::SpellList),
        }
        .map(move |message| Message::ViewUpdated(self.active_view, message));

        let constrained_content = widget::container(view)
            .max_width(VIEW_WIDTH)
            .padding(Padding::new(0.0).vertical(10.0));

        let centering_container = widget::container(constrained_content).center_x(Length::Fill);

        let scrollable_container = widget::scrollable(centering_container)
            .width(Length::Fill)
            .height(Length::Fill);

        column![overview, divider, scrollable_container]
            .align_x(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    /// Update the `View` of `id` with the provided `message`.
    ///
    /// # Notes
    ///
    /// If the provided `message` does not correspond to a valid `id`, or the type of `message`
    /// is not valid for the type of `View` that `id` specifies, this method is idempotent.
    pub fn update_view(&mut self, id: ViewId, message: ViewMessage) -> Task<Message> {
        let Some(view) = self.view_mut(id) else {
            tracing::error!("could not find view with id '{id:?}'");

            return Task::none();
        };

        match message {
            ViewMessage::Dashboard(dashboard_message) => {
                let View::Dashboard(dashboard) = view else {
                    tracing::error!(
                        "view with id '{id:?}' does not match message of type '{dashboard_message:?}'"
                    );

                    return Task::none();
                };

                let (child_task, maybe_effect) = dashboard.update(dashboard_message);

                let mut tasks = Vec::with_capacity(2);
                tasks.push(map_task(child_task, id, ViewMessage::Dashboard));

                if let Some(effect) = maybe_effect {
                    match effect {
                        DashboardEffect::OpenView(request) => {
                            let task = Task::done(Message::OpenView(request));
                            tasks.push(task);
                        }
                    }
                }

                Task::batch(tasks)
            }
            ViewMessage::SpellForm(spell_form_message) => {
                let View::SpellForm(spell_form) = view else {
                    tracing::error!(
                        "view with id '{id:?}' does not match message of type '{spell_form_message:?}'"
                    );

                    return Task::none();
                };

                let (child_task, maybe_effect) = spell_form.update(spell_form_message);

                let mapped_task = map_task(child_task, id, ViewMessage::SpellForm);

                if let Some(spell_form_effect) = maybe_effect {
                    match spell_form_effect {
                        SpellFormEffect::LoadFailed(err) => {
                            panic!("{err:?}");
                            // self.remove(tab_id);
                            //
                            // effect = Some(TabManagerEffect::LoadFailed(error));
                        }
                    }
                }

                mapped_task
            }
            ViewMessage::SpellList(spell_list_message) => {
                let View::SpellList(spell_list) = view else {
                    tracing::error!(
                        "view with id '{id:?}' does not match message of type '{spell_list_message:?}'"
                    );

                    return Task::none();
                };

                let (child_task, maybe_effect) = spell_list.update(spell_list_message);

                let mut tasks = Vec::with_capacity(2);
                tasks.push(map_task(child_task, id, ViewMessage::SpellList));

                if let Some(effect) = maybe_effect {
                    match effect {
                        SpellListEffect::OpenView(request) => {
                            let task = Task::done(Message::OpenView(request));
                            tasks.push(task);
                        }
                    }
                }

                Task::batch(tasks)
            }
        }
    }

    /// Open the `View` defined by `request`.
    ///
    /// Depending on the type of `View` the `request` contains,
    /// this may instead focus an existing instance of `View`.
    pub fn open_view(&mut self, request: Request) -> Task<Message> {
        tracing::debug!("opening view: {request:?}");

        match request {
            Request::SpellForm { mode } => {
                let id = ViewId::unique();
                let (spell_form, task) = SpellForm::new(mode, self.context.clone());
                let tab = View::SpellForm(Box::new(spell_form));
                self.views.push((id, tab));
                self.active_view = id;

                let mapped_task = map_task(task, id, ViewMessage::SpellForm);

                return mapped_task;
            }
            Request::SpellList => {
                // Check if view already exists
                let Some(id) = self.view_exists(|view| matches!(view, View::SpellList(_))) else {
                    let id = ViewId::unique();
                    let new_tab = View::SpellList(Box::new(SpellList::new()));
                    self.views.push((id, new_tab));
                    self.active_view = id;

                    return Task::none();
                };

                self.active_view = id;
            }
        }

        Task::none()
    }

    /// Close the `View` of the provided `close_id`.
    pub fn close_view(&mut self, close_id: ViewId) -> Task<Message> {
        tracing::debug!("closing view: {close_id:?}");

        // Only recalculate the active view if we are closing the focused one
        if self.active_view == close_id {
            tracing::debug!("closed active view");

            // If any step fails, this just defaults to Dashboard view.
            self.active_view = self
                .views
                .iter()
                .position(|(id, _)| *id == close_id)
                .and_then(|index| {
                    // Default to revert to prior view.
                    if index > 0 {
                        self.views.get(index - 1)
                    }
                    // If not prior, try next view.
                    else {
                        self.views.get(index + 1)
                    }
                })
                .map(|(id, _)| *id)
                .unwrap_or(self.dashboard_id);
        }

        // Close the view.
        self.views.retain(|(id, _)| *id != close_id);

        Task::none()
    }

    pub fn focus_view(&mut self, view_id: ViewId) -> Task<Message> {
        self.active_view = view_id;

        Task::none()
    }

    fn overview<'a>(&'a self, views: &'a [(ViewId, View)], active: ViewId) -> Element<'a, Message> {
        let mut bar = Row::with_capacity(views.len());

        for (id, view) in views {
            let text = {
                let text = components::text::display(view.title());
                widget::container(text).clip(true).padding(5)
            };

            let space = widget::space::horizontal().width(Length::Fill);

            let button: Element<'_, Message> = if *id != self.dashboard_id {
                let icon = components::icon(IconName::Close).class(SvgClass::Normal);
                widget::button(icon)
                    .height(OVERVIEW_BAR_HEIGHT)
                    .on_press(Message::CloseView(*id))
                    .into()
            } else {
                widget::space().into()
            };

            let bar_element = row![text, space, button]
                .height(OVERVIEW_BAR_HEIGHT)
                .width(OVERVIEW_ELEMENT_WIDTH)
                .align_y(Alignment::Center);

            let mut container = widget::container(bar_element);

            if *id == active {
                container = container.class(ContainerClass::Surface);
            } else {
                container = container.class(ContainerClass::Ghost);
            }

            let ma = widget::mouse_area(container).on_press(Message::FocusView(*id));

            bar = bar.push(ma);

            let divider = widget::container(widget::rule::vertical(1)).center_y(30);

            bar = bar.push(divider);
        }

        widget::scrollable(bar)
            .direction(Direction::Horizontal(Scrollbar::hidden()))
            .width(Length::Fill)
            .into()
    }

    /// Get a reference to the `View` associated with `id`, if it exists.
    fn view(&self, id: ViewId) -> Option<&View> {
        self.views
            .iter()
            .find(|(view_id, _)| *view_id == id)
            .map(|(_, view)| view)
    }

    /// Get a mutable reference to the `View` associated with `id`, if it exists.
    fn view_mut(&mut self, id: ViewId) -> Option<&mut View> {
        self.views
            .iter_mut()
            .find(|(view_id, _)| *view_id == id)
            .map(|(_, view)| view)
    }

    /// Check if a `View` exists that upholds the given predicate.
    fn view_exists(&self, predicate: impl Fn(&View) -> bool) -> Option<ViewId> {
        self.views
            .iter()
            .find(|(_, view)| predicate(view))
            .map(|(id, _)| *id)
    }
}

/// Wrap a given given generic `Task<M>` in the provided `ViewMessage` wrapper.
///
/// This is a helper function that maps output tasks from a `View`, to its specific message type
/// variant in `ViewMessage`, as this is a common pattern for each `View`.
fn map_task<M>(
    task: Task<M>,
    view_id: ViewId,
    wrapper: impl Fn(M) -> ViewMessage + Send + Sync + 'static,
) -> Task<Message>
where
    M: Send + Sync + 'static,
{
    task.map(move |message| Message::ViewUpdated(view_id, wrapper(message)))
}
