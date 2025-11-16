use std::ops::{Index, IndexMut};

use gpui::{
    BorrowAppContext, Context, InteractiveElement, IntoElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, px,
};
use gpui_component::{
    Icon, IconName, Side, StyledExt,
    sidebar::{Sidebar, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem},
};
use state::AppState;
use strum::EnumCount;
use types::{FormMode, View};

#[derive(PartialEq, Clone, Copy, EnumCount)]
enum NavigationItem {
    Encounters,
    Monsters,
    MonstersTypes,
    Spells,
    SpellsLevels,
    SpellsSchools,
}

impl Index<NavigationItem> for [bool; NavigationItem::COUNT] {
    type Output = bool;

    fn index(&self, index: NavigationItem) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<NavigationItem> for [bool; NavigationItem::COUNT] {
    fn index_mut(&mut self, index: NavigationItem) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

pub struct NavigationMenu {
    collapsed: bool,
    items: [bool; NavigationItem::COUNT],
}

impl NavigationMenu {
    pub fn new() -> Self {
        Self {
            collapsed: false,
            items: [false; NavigationItem::COUNT],
        }
    }

    /// Returns the current state of collapse.
    pub fn collapsed(&self) -> bool {
        self.collapsed
    }

    /// Toggles the collapse state.
    pub fn collapse(&mut self) {
        self.collapsed = !self.collapsed;
    }

    /// Returns true if `item` is currently active.
    fn is_active(&self, item: NavigationItem) -> bool {
        if self.collapsed {
            return false;
        }

        self.items[item]
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "index is guaranteed to be safe because of enum discriminant values"
    )]
    /// Toggles the state of `item`.
    fn toggle(&mut self, item: NavigationItem) {
        if self.collapsed {
            return;
        }

        self.items[item] = !self.items[item]
    }

    fn spells(&self, cx: &mut Context<Self>) -> SidebarMenuItem {
        let spell_levels_active = self.is_active(NavigationItem::SpellsLevels);
        let spell_levels_submenu = SidebarMenuItem::new("Levels")
            .active(spell_levels_active)
            .on_click(cx.listener(|nav, _, _, _| {
                nav.toggle(NavigationItem::SpellsLevels);
            }))
            .children([
                SidebarMenuItem::new("1st"),
                SidebarMenuItem::new("2nd"),
                SidebarMenuItem::new("3rd"),
                SidebarMenuItem::new("4th"),
                SidebarMenuItem::new("5th"),
                SidebarMenuItem::new("6th"),
                SidebarMenuItem::new("7th"),
                SidebarMenuItem::new("8th"),
                SidebarMenuItem::new("9th"),
            ]);

        let spell_schools_active = self.is_active(NavigationItem::SpellsSchools);
        let spell_schools_submenu = SidebarMenuItem::new("Schools")
            .active(spell_schools_active)
            .on_click(cx.listener(|nav, _, _, _| {
                nav.toggle(NavigationItem::SpellsSchools);
            }))
            .children([
                SidebarMenuItem::new("Abjuration"),
                SidebarMenuItem::new("Conjuration"),
                SidebarMenuItem::new("Divination"),
                SidebarMenuItem::new("Enchantment"),
                SidebarMenuItem::new("Evocation"),
                SidebarMenuItem::new("Illusion"),
                SidebarMenuItem::new("Necromancy"),
                SidebarMenuItem::new("Transmutation"),
            ]);

        let create = SidebarMenuItem::new("Create new")
            .suffix(Icon::new(IconName::Plus))
            .on_click(cx.listener(|_, _, _, cx| {
                cx.update_global(|state: &mut AppState, _| {
                    state.view = View::SpellForm {
                        mode: FormMode::Create,
                    }
                })
            }));

        let spells_active = self.is_active(NavigationItem::Spells);
        SidebarMenuItem::new("Spells")
            .icon(IconName::Building2)
            .active(spells_active)
            .on_click(cx.listener(|nav, _, _, _| nav.toggle(NavigationItem::Spells)))
            .children([create, spell_schools_submenu, spell_levels_submenu])
    }

    fn monsters(&mut self, cx: &mut Context<Self>) -> SidebarMenuItem {
        let monsters_active = self.is_active(NavigationItem::Monsters);

        let monster_types_active = self.is_active(NavigationItem::MonstersTypes);
        let monsters_types_submenu = SidebarMenuItem::new("Types")
            .active(monster_types_active)
            .on_click(cx.listener(|nav, _, _, _| {
                nav.toggle(NavigationItem::MonstersTypes);
            }))
            .children([
                SidebarMenuItem::new("Aberration"),
                SidebarMenuItem::new("Beast"),
                SidebarMenuItem::new("Celestial"),
                SidebarMenuItem::new("Construct"),
                SidebarMenuItem::new("Dragon"),
                SidebarMenuItem::new("Elemental"),
                SidebarMenuItem::new("Fey"),
                SidebarMenuItem::new("Fiend"),
                SidebarMenuItem::new("Giant"),
                SidebarMenuItem::new("Humanoid"),
                SidebarMenuItem::new("Monstrosity"),
                SidebarMenuItem::new("Ooze"),
                SidebarMenuItem::new("Plant"),
                SidebarMenuItem::new("Undead"),
            ]);

        let create = SidebarMenuItem::new("Create new").suffix(Icon::new(IconName::Plus));

        SidebarMenuItem::new("Monsters")
            .icon(IconName::CaseSensitive)
            .active(monsters_active)
            .on_click(cx.listener(|nav, _, _, _| nav.toggle(NavigationItem::Monsters)))
            .children([create, monsters_types_submenu])
    }

    fn encounters(&mut self, cx: &mut Context<Self>) -> SidebarMenuItem {
        let encounters_active = self.is_active(NavigationItem::Encounters);

        let create = SidebarMenuItem::new("Create new").suffix(Icon::new(IconName::Plus));

        SidebarMenuItem::new("Encounters")
            .icon(IconName::Settings)
            .active(encounters_active)
            .on_click(cx.listener(|nav, _, _, _| nav.toggle(NavigationItem::Encounters)))
            .children([create])
    }
}

impl Render for NavigationMenu {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let header = SidebarHeader::new().child(
            div()
                .id("navigation-header")
                .size_full()
                .h_flex()
                .gap_2()
                .child(Icon::new(IconName::WindowRestore))
                .when(!self.collapsed, |this| this.child("Initiative"))
                .on_click(cx.listener(|_, _, _, cx| {
                    cx.update_global(|state: &mut AppState, _| state.view = View::Index)
                })),
        );

        let tools = SidebarGroup::new("Tools").child(SidebarMenu::new().children([
            self.encounters(cx),
            self.monsters(cx),
            self.spells(cx),
        ]));

        Sidebar::new(Side::Left)
            .width(px(300.0))
            .header(header)
            .child(tools)
            .collapsible(true)
            .collapsed(self.collapsed)
    }
}
