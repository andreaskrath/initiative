use std::ops::{Index, IndexMut};

use strum::EnumCount;

use crate::View;

#[derive(Debug, Clone)]
pub enum NavigationMessage {
    ToggleCollapse,
    ToggleItem(ExpandableNavigationItemId),
    Navigate(View),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, EnumCount)]
pub enum ExpandableNavigationItemId {
    Spell,
    SpellSchool,
    SpellLevel,
}

impl Index<ExpandableNavigationItemId> for [bool; ExpandableNavigationItemId::COUNT] {
    type Output = bool;

    fn index(&self, index: ExpandableNavigationItemId) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<ExpandableNavigationItemId> for [bool; ExpandableNavigationItemId::COUNT] {
    fn index_mut(&mut self, index: ExpandableNavigationItemId) -> &mut Self::Output {
        &mut self[index as usize]
    }
}
