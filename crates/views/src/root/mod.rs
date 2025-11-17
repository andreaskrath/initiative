mod navigation;

use components::hr;
use gpui::{
    AnyView, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, px,
};

use gpui_component::{
    ActiveTheme, Icon, IconName,
    breadcrumb::{Breadcrumb, BreadcrumbItem},
    button::{Button, ButtonVariant, ButtonVariants},
    h_flex, v_flex,
};
use navigation::NavigationMenu;
use state::AppState;
use tracing::debug;
use types::View;

use crate::{IndexView, SpellFormView};

struct TypedAnyView {
    view: AnyView,
    variant: View,
}

impl TypedAnyView {
    fn new(view: AnyView, variant: View) -> Self {
        Self { view, variant }
    }

    fn variant(&self) -> &View {
        &self.variant
    }

    fn view(&self) -> AnyView {
        self.view.clone()
    }
}

pub struct RootView {
    navigation_menu: Entity<NavigationMenu>,
    view: TypedAnyView,
}

impl RootView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let navigation_menu = cx.new(|_| NavigationMenu::new());

        // Register detached subscription for `AppState` updates.
        cx.observe_global_in::<AppState>(window, |root, window, cx| root.update_view(window, cx))
            .detach();

        let view_type = cx.global::<AppState>().view.clone();
        let any_view = cx.new(|_| IndexView::new()).into();
        let view = TypedAnyView::new(any_view, view_type);

        Self {
            navigation_menu,
            view,
        }
    }

    fn update_view(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let view = cx.global::<AppState>().view.clone();

        debug!(
            "checking if view should be updated from {:?} to {:?}",
            self.view.variant(),
            view
        );
        if view == *self.view.variant() {
            debug!("view is not updated");
            return;
        }

        debug!("view is updated to {:?}", view);
        let view_type = view.clone();
        let new_view = match view {
            View::Index => cx.new(|_| IndexView::new()).into(),
            View::SpellForm { mode } => cx.new(|cx| SpellFormView::new(window, cx, mode)).into(),
        };

        self.view = TypedAnyView::new(new_view, view_type);
    }
}

impl Render for RootView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_collapsed = self.navigation_menu.read(cx).collapsed();
        let navigation_button_icon = match is_collapsed {
            true => Icon::new(IconName::PanelLeftOpen),
            false => Icon::new(IconName::PanelLeftClose),
        }
        .size_6();
        let navigation_expand_button = Button::new("navigation-button")
            .w(px(44.0))
            .with_variant(ButtonVariant::Link)
            .child(navigation_button_icon)
            .on_click(cx.listener(|root, _, _, cx| {
                root.navigation_menu.update(cx, |nav, _| nav.collapse())
            }));
        let topbar = h_flex()
            .gap_2()
            .p_0p5()
            .bg(cx.theme().sidebar)
            .child(navigation_expand_button)
            .child(Breadcrumb::new().children([
                BreadcrumbItem::new("Item 1"),
                BreadcrumbItem::new("Item 2"),
                BreadcrumbItem::new("Item 3"),
                BreadcrumbItem::new("Item 4"),
                BreadcrumbItem::new("Item 5"),
            ]));

        let content_area = h_flex()
            .flex_1()
            .min_h_0()
            .child(self.navigation_menu.clone())
            .child(self.view.view());

        v_flex()
            .w_full()
            .h_full()
            .child(topbar)
            .child(hr(cx.theme()))
            .child(content_area)
    }
}
