mod navigation;

use components::hr;
use gpui::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};

use gpui_component::{
    ActiveTheme, Icon, IconName, StyledExt,
    breadcrumb::{Breadcrumb, BreadcrumbItem},
    button::{Button, ButtonVariant, ButtonVariants},
    h_flex, v_flex,
};
use navigation::NavigationMenu;

pub struct RootView {
    navigation_menu: Entity<NavigationMenu>,
}

impl RootView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let navigation_menu = cx.new(|_| NavigationMenu::new());

        Self { navigation_menu }
    }
}

impl Render for RootView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let is_collapsed = self.navigation_menu.read(cx).collapsed();
        let navigation_button_icon = match is_collapsed {
            true => Icon::new(IconName::PanelLeftOpen),
            false => Icon::new(IconName::PanelLeftClose),
        }
        .size_6();
        let navigation_expand_button = Button::new("navigation-button")
            .with_variant(ButtonVariant::Ghost)
            .child(navigation_button_icon)
            .on_click(cx.listener(|root, _, _, cx| {
                root.navigation_menu.update(cx, |nav, _| nav.collapse())
            }))
            .size_8();
        let topbar = h_flex().gap_2().child(navigation_expand_button).child(
            Breadcrumb::new()
                .item(BreadcrumbItem::new("breadcrumb-item-1", "Item 1"))
                .item(BreadcrumbItem::new("breadcrumb-item-2", "Item 2"))
                .item(BreadcrumbItem::new("breadcrumb-item-3", "Item 3"))
                .item(BreadcrumbItem::new("breadcrumb-item-4", "Item 4"))
                .item(BreadcrumbItem::new("breadcrumb-item-5", "Item 5")),
        );

        let vertical_split = v_flex()
            .size_full()
            .children([div().p(px(5.0)).child(topbar).bg(theme.sidebar), hr(theme)]);

        div()
            .w_full()
            .h_full()
            .h_flex()
            .child(self.navigation_menu.clone())
            .child(vertical_split)
    }
}
